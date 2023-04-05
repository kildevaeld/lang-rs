use proc_macro2::{Group, TokenStream};
use syn::{
    parse::{Parse, Parser},
    Block, Ident, LitStr, Token, Type,
};

mod kw {
    syn::custom_keyword!(rule);
}

pub fn parse(tokens: TokenStream) -> syn::Result<Pratt> {
    Pratt::parse.parse2(tokens)
}

#[derive(Debug)]
pub struct Pratt {
    pub module_name: Ident,
    pub return_type: Type,
    pub rules: Vec<RuleGroup>,
}

impl Parse for Pratt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let module_name = input.parse::<Ident>()?;
        let _ = input.parse::<Token![->]>()?;
        let return_type = input.parse::<Type>()?;

        let mut rules = Vec::default();

        while !input.is_empty() {
            rules.push(input.parse()?);

            eprintln!("GFDFFDDFD");

            if input.peek(Token![-]) && input.peek2(Token![-]) {
                input.parse::<Token![-]>()?;
                input.parse::<Token![-]>()?;
            } else {
                break;
            }
        }

        Ok(Pratt {
            module_name,
            return_type,
            rules,
        })
    }
}

#[derive(Debug)]
pub struct RuleGroup {
    pub rules: Vec<Rule>,
}

impl Parse for RuleGroup {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rules = Vec::default();

        while !input.is_empty() {
            rules.push(input.parse()?);

            if !input.peek(kw::rule) {
                break;
            }
        }

        Ok(RuleGroup { rules })
    }
}

#[derive(Debug)]
pub struct Rule {
    pub atoms: Vec<Atom>,
    pub action: Option<Block>,
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if !input.peek(kw::rule) {
            return Err(input.error("expected rule"));
        }

        let mut atoms = Vec::default();

        while !input.is_empty() {
            // If we are hitting af brace (start of action), a new rule or a "or" we break
            if input.peek(syn::token::Brace) || input.peek(kw::rule) || input.peek(Token![/]) {
                println!("DSDADADSADASD");
                break;
            }

            atoms.push(input.parse()?);
        }

        let action = if input.peek(syn::token::Brace) {
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Rule { atoms, action })
    }
}

#[derive(Debug)]
pub enum Atom {
    Prec,
    Parser { name: Ident },
    Rule { rule: Rule },
    Token(AtomToken),
}

#[derive(Debug)]
pub struct AtomToken {
    pub name: LitStr,
}

impl Parse for Atom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rule = if input.peek(Token![@]) {
            let _ = input.parse::<Token![@]>()?;
            Atom::Prec
        } else if input.peek(Ident) {
            let name = input.parse::<Ident>()?;
            Atom::Parser { name }
        } else if input.peek(syn::token::Paren) {
            let group = input.parse::<Group>()?;

            let rule = Rule::parse.parse2(group.stream())?;

            Atom::Rule { rule }
        } else if input.peek(LitStr) {
            Atom::Token(AtomToken {
                name: input.parse()?,
            })
        } else {
            //println!("found: {:?}", input);
            return Err(input.error(format!("expected rule item: {:?}", input)));
        };

        Ok(rule)
    }
}
