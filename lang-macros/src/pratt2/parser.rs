use proc_macro2::{Group, TokenStream};
use proc_macro2::{Punct, Spacing};
use quote::quote;
use quote::ToTokens;
use syn::{
    ext::IdentExt,
    parse::{Parse, Parser},
    token::Paren,
    Block, ExprBlock, Ident, LitStr, Token, Type,
};

pub fn parse(stream: TokenStream) -> syn::Result<Bundle> {
    Bundle::parse.parse2(stream)
}

#[derive(Debug, Clone)]
pub struct Bundle {
    pub module: Ident,
    pub return_type: Type,
    pub rule_list: Vec<RuleGroup>,
}

impl Parse for Bundle {
    fn parse(stream: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name = stream.parse::<Ident>()?;
        let _ = stream.parse::<Token![->]>()?;
        let ret_name = stream.parse::<Type>()?;

        let mut precedence_list = Vec::default();
        loop {
            if stream.is_empty() {
                break;
            }

            let rule_list = stream.parse::<RuleGroup>()?;

            precedence_list.push(rule_list);

            if stream.is_empty() {
                break;
            }

            let _ = stream.parse::<Token![-]>()?;
            let _ = stream.parse::<Token![-]>()?;
        }

        Ok(Bundle {
            module: fn_name,
            return_type: ret_name,
            rule_list: precedence_list,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub items: Vec<RuleEntry>,
    pub map: Option<Block>,
}

#[derive(Debug, Clone)]
pub enum RuleEntry {
    Single(RuleItem),
    Alternatives(Vec<RuleItem>),
}

impl RuleEntry {
    pub fn is_prefix(&self) -> bool {
        let item = match self {
            Self::Single(s) => s,
            Self::Alternatives(s) => match s.first() {
                Some(s) => s,
                None => return false,
            },
        };

        !matches!(item.kind, RuleItemKind::Prec)
    }

    pub fn peek(&self) -> Option<TokenStream> {
        match self {
            RuleEntry::Single(s) => s.peek(),
            RuleEntry::Alternatives(s) => {
                let stream = s.iter().filter_map(|m| m.peek());
                Some(quote!(
                    #(#stream)||*
                ))
            }
        }
    }
}

impl Parse for RuleEntry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items = input.parse::<RuleItem>()?;
        let mut or_items = Vec::default();
        while input.peek(Token![/]) {
            let _ = input.parse::<Token![/]>()?;
            or_items.push(input.parse()?);
        }

        let entry = if !or_items.is_empty() {
            let mut items = vec![items];
            items.extend(or_items);
            RuleEntry::Alternatives(items)
        } else {
            RuleEntry::Single(items)
        };

        Ok(entry)
    }
}

impl Rule {
    pub fn peek(&self) -> Option<TokenStream> {
        match self.items.iter().find(|m| m.is_prefix()) {
            Some(m) => m.peek(),
            None => None,
        }
    }

    pub fn is_prefix(&self) -> bool {
        self.items
            .first()
            .map(|m| m.is_prefix())
            .unwrap_or_default()
    }
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::default();

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(syn::token::Brace) {
                break;
            }

            items.push(input.parse()?);
        }

        let map = if input.peek(syn::token::Brace) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Rule { items, map })
    }
}

#[derive(Debug, Clone)]
pub enum RuleItemKind {
    Prec,
    Parser { name: Ident },
    Rule { rule: Rule },
    Token { name: Token },
}

impl RuleItemKind {
    pub fn to_parse(&self, prefix: bool) -> TokenStream {
        match self {
            RuleItemKind::Parser { name } => {
                quote!(input.parse::<#name>()?)
            }
            RuleItemKind::Prec => quote!(__expression(input, 0)?),
            RuleItemKind::Rule { rule } => rule.parse_token(prefix),
            RuleItemKind::Token { name } => {
                quote!(input.parse::<#name>()?)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub name: LitStr,
}

impl ToTokens for Token {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let str = self.name.value();
        if str.chars().all(|m| m.is_ascii_punctuation()) {
            let mut puncts = Vec::new();

            let count = str.chars().count();

            for (idx, char) in str.chars().enumerate() {
                let spacing = if idx + 1 == count {
                    Spacing::Alone
                } else {
                    Spacing::Joint
                };
                puncts.push(Punct::new(char, spacing));
            }
            quote!(Token![#(#puncts)*])
        } else {
            quote!(Token![#str])
        }
        .to_tokens(tokens)
    }
}

impl Parse for RuleItemKind {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rule = if input.peek(Token![@]) {
            let _ = input.parse::<Token![@]>()?;
            RuleItemKind::Prec
        } else if input.peek(Ident) {
            let name = input.parse::<Ident>()?;
            RuleItemKind::Parser { name }
        } else if input.peek(Paren) {
            let group = input.parse::<Group>()?;

            let rule = Rule::parse.parse2(group.stream())?;

            RuleItemKind::Rule { rule }
        } else if input.peek(LitStr) {
            RuleItemKind::Token {
                name: Token {
                    name: input.parse()?,
                },
            }
        } else {
            //println!("found: {:?}", input);
            return Err(input.error(format!("expected rule item: {:?}", input)));
        };

        Ok(rule)
    }
}

#[derive(Debug, Clone)]
pub struct RuleItem {
    pub kind: RuleItemKind,
    pub name: Option<Ident>,
    pub negation: bool,
}

impl RuleItem {
    pub fn peek(&self) -> Option<TokenStream> {
        match &self.kind {
            RuleItemKind::Parser { name } => Some(quote!(input.peek::<#name>())),
            RuleItemKind::Prec => None,
            RuleItemKind::Rule { rule } => rule.peek(),
            RuleItemKind::Token { name } => Some(quote!(input.peek::<#name>())),
        }
    }
}

impl Parse for RuleItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = if input.peek(syn::Ident::peek_any) && input.peek2(Token![:]) {
            let name = input.parse::<Ident>()?;
            let _ = input.parse::<Token![:]>()?;
            Some(name)
        } else {
            None
        };

        let negation = if input.peek(Token![!]) {
            let _ = input.parse::<Token![!]>();
            true
        } else {
            false
        };

        let kind = input.parse()?;

        Ok(RuleItem {
            kind,
            name,
            negation,
        })
    }
}

pub struct MapBlock {
    pub block: ExprBlock,
}

impl Parse for MapBlock {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MapBlock {
            block: input.parse()?,
        })
    }
}

#[derive(Debug, Clone)]

pub struct RuleGroup {
    pub rules: Vec<Rule>,
}

impl Parse for RuleGroup {
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

        Ok(RuleGroup { rules })
    }
}
