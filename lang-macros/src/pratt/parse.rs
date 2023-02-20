use proc_macro2::{Group, Ident, Punct};
use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream, Parser},
    Block, Ident as IdentPeek, LitStr, Token,
};

#[derive(Clone, Debug)]
pub enum Operator {
    Punct(Vec<Punct>),
    Literal(LitStr),
}

impl ToTokens for Operator {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Operator::Literal(lit) => quote!(Token![#lit]).to_tokens(tokens),
            Operator::Punct(punct) => quote!(Token![#(#punct)*]).to_tokens(tokens),
        }
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Literal(lit) => {
                write!(f, "{}", lit.value())
            }
            Operator::Punct(punct) => {
                let i = punct.iter().map(|m| m.as_char()).collect::<String>();
                write!(f, "{i}")
            }
        }
    }
}

impl Parse for Operator {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            Ok(Operator::Literal(input.parse()?))
        } else if input.peek(syn::token::Bracket) {
            let group = input.parse::<Group>()?;
            Ok(Operator::Punct(parse_puncts.parse2(group.stream())?))
        } else {
            Err(input.error("invalid operator"))
        }
    }
}

impl PartialEq for Operator {
    fn eq(&self, other: &Self) -> bool {
        use Operator::*;
        match (self, other) {
            (Literal(a), Literal(b)) => a == b,
            (Punct(a), Punct(b)) => {
                if a.len() != b.len() {
                    return false;
                }

                for i in 0..a.len() {
                    if a[i].as_char() != b[i].as_char() {
                        return false;
                    }
                }

                true
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(Operator),
    Any(Vec<Operator>),
    Ident(Ident),
}

fn parse_operators(input: ParseStream) -> syn::Result<Vec<Operator>> {
    let mut out = Vec::default();

    loop {
        if input.is_empty() {
            break;
        }

        out.push(input.parse()?);
    }

    Ok(out)
}

fn parse_puncts(input: ParseStream) -> syn::Result<Vec<Punct>> {
    let mut out = Vec::default();

    loop {
        if input.is_empty() {
            break;
        }

        out.push(input.parse()?);
    }

    Ok(out)
}

impl Parse for Token {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let token = if input.peek(syn::token::Paren) {
            let group = input.parse::<Group>()?;
            Token::Any(parse_operators.parse2(group.stream())?)
        } else if input.peek(IdentPeek) {
            let ident = input.parse()?;
            Token::Ident(ident)
        } else {
            Token::Operator(input.parse()?)
        };

        Ok(token)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuleItem {
    Prec { name: Ident },
    Parse { name: Ident, parse: Token },
    Token(Token),
}

impl RuleItem {
    pub fn is_prec(&self) -> bool {
        matches!(self, RuleItem::Prec { .. })
    }
}

impl Parse for RuleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rule = if input.peek(syn::Ident::peek_any) && input.peek2(Token![:]) {
            if input.peek3(Token![@]) {
                let name = input.parse::<Ident>()?;
                let _ = input.parse::<Token![:]>()?;
                let _ = input.parse::<Token![@]>()?;
                RuleItem::Prec { name }
            } else {
                let name = input.parse::<Ident>()?;
                let _ = input.parse::<Token![:]>()?;
                let parse = input.parse::<Token>()?;
                RuleItem::Parse { name, parse }
            }
        } else {
            RuleItem::Token(input.parse::<Token>()?)
        };

        Ok(rule)
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub items: Vec<RuleItem>,
    pub action: Block,
}

impl Rule {
    pub fn is_infix(&self) -> bool {
        let first = match self.items.first() {
            Some(first) => first,
            None => return false,
        };

        let last = self.items.last().unwrap();

        if last == first {
            false
        } else {
            (first.is_prec() && last.is_prec() && self.items.len() == 3)
                || first.is_prec() && self.items.len() > 1
        }
    }
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::default();
        let mut action = None;

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(syn::token::Brace) {
                action = Some(input.parse::<Block>()?);
                break;
            }

            let item = input.parse::<RuleItem>()?;

            items.push(item);
        }

        let action = match action {
            None => return Err(input.error("no action")),
            Some(action) => action,
        };

        Ok(Rule { items, action })
    }
}

#[derive(Debug, Clone)]

pub struct RuleList {
    pub rules: Vec<Rule>,
}

impl Parse for RuleList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rules = Vec::default();

        loop {
            if input.is_empty() {
                break;
            }

            let rule = input.parse::<Rule>()?;

            rules.push(rule);

            if input.peek(Token![-]) && input.peek2(Token![-]) {
                break;
            }
        }

        Ok(RuleList { rules })
    }
}
