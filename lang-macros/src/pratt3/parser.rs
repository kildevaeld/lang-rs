use darling::ToTokens;
use proc_macro2::{Group, Punct, Spacing, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
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

            if input.peek(Token![-]) && input.peek2(Token![-]) {
                let _ = input.parse::<Token![-]>()?;
                let _ = input.parse::<Token![-]>()?;
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
    pub rules: Vec<Precedence>,
}

// impl RuleGroup {
//     pub fn peek(&self) -> TokenStream {
//         let peek = self.rules.iter().map(|m| m.peek());
//         quote!(
//             #(#peek)||*
//         )
//     }
// }

impl Parse for RuleGroup {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rules = Vec::default();

        while !input.is_empty() {
            let _ = input.parse::<kw::rule>()?;
            rules.push(input.parse()?);

            if !input.peek(kw::rule) {
                break;
            }
        }

        Ok(RuleGroup { rules })
    }
}

#[derive(Debug)]
pub struct Precedence {
    pub rules: Vec<Rule>,
}

impl Parse for Precedence {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut rules = Vec::default();
        while !input.is_empty() {
            let rule = input.parse()?;
            rules.push(rule);
            if input.peek(Token![/]) {
                let _ = input.parse::<Token![/]>()?;
            } else {
                break;
            }
        }

        Ok(Precedence { rules })
    }
}

// impl Precedence {
//     pub fn peek(&self) -> TokenStream {
//         let iter = self.rules.iter().map(|item| item.peek());
//         quote!(
//             #(#iter)||*
//         )
//     }
// }

#[derive(Debug)]
pub struct Rule {
    pub items: Vec<Expr>,
    pub action: Option<Block>,
}

impl Rule {
    pub fn peek(&self) -> TokenStream {
        let iter: Box<dyn Iterator<Item = _>> = if self.is_prefix() {
            Box::new(self.items.iter())
        } else {
            Box::new(self.items.iter().skip_while(|item| item.is_prec()))
        };

        let iter = iter
            .take_while(|item| !item.is_prec())
            .map(|item| item.peek())
            .collect::<Vec<_>>();

        if iter.len() == 1 {
            quote!(
               #(#iter)&&*
            )
        } else {
            quote!(
               ( #(#iter)&&*)
            )
        }
    }

    pub fn is_prefix(&self) -> bool {
        !self.items[0].is_prec()
    }

    pub fn build_parse(&self, level: u8) -> TokenStream {
        let parse = self
            .items
            .iter()
            .skip(if self.is_prefix() { 0 } else { 1 })
            .filter_map(|expr| match expr {
                Expr::Named { name, atom } => {
                    let parse = atom.create_parse(level);
                    Some(quote!(
                        let #name = #parse?;
                    ))
                }
                Expr::UnNamed { atom } => {
                    let parse = atom.create_parse(level);
                    Some(quote!(
                        let _ = #parse?;
                    ))
                }
                Expr::Not { .. } => None,
            });

        let peek = self.peek();

        let first = if !self.is_prefix() {
            let first = self.items.first().expect("first item");
            if let Expr::Named { name, atom } = first {
                Some(quote!(
                    let #name = left;
                ))
            } else {
                None
            }
        } else {
            None
        };

        let action = if let Some(action) = &self.action {
            quote!(
                #first
                #(#parse)*
                #action
            )
        } else {
            quote!(
                {
                    (
                        #first
                        #(#parse),*
                    )
                }
            )
        };

        quote!(
            if #peek {
                #action
            }
        )
    }
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut items = Vec::default();
        let mut action = None;

        while !input.is_empty() {
            let item = input.parse::<Expr>()?;
            items.push(item);

            if input.peek(syn::token::Brace) {
                action = Some(input.parse::<Block>()?);
                break;
            } else if input.peek(Token![/])
                || input.peek(kw::rule)
                || (input.peek(Token![-]) && input.peek2(Token![-]))
            {
                break;
            }
        }

        Ok(Rule { items, action })
    }
}

#[derive(Debug)]
pub enum Expr {
    Named { name: Ident, atom: Atom },
    UnNamed { atom: Atom },
    Not { atom: Atom },
}

impl Expr {
    pub fn peek(&self) -> TokenStream {
        match self {
            Expr::Named { atom, .. } => atom.peek(),
            Expr::Not { atom } => {
                let peek = atom.peek();
                quote!(!#peek)
            }
            Expr::UnNamed { atom } => atom.peek(),
        }
    }

    pub fn atom(&self) -> &Atom {
        match self {
            Expr::Named { atom, .. } => atom,
            Expr::Not { atom } => atom,
            Expr::UnNamed { atom } => atom,
        }
    }

    pub fn is_prec(&self) -> bool {
        match self.atom() {
            Atom::Prec => true,
            _ => false,
        }
    }
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let out = if input.peek(Token!(!)) {
            let _ = input.parse::<Token![!]>()?;
            Expr::Not {
                atom: input.parse()?,
            }
        } else if input.peek(syn::Ident) && input.peek2(Token![:]) {
            let name = input.parse()?;
            let _ = input.parse::<Token![:]>()?;
            Expr::Named {
                name,
                atom: input.parse()?,
            }
        } else {
            Expr::UnNamed {
                atom: input.parse()?,
            }
        };

        Ok(out)
    }
}

#[derive(Debug)]
pub enum Atom {
    Prec,
    Parser { name: Ident },
    Token(AtomToken),
    Rule(Vec<Rule>),
}

impl Atom {
    pub fn peek(&self) -> TokenStream {
        match self {
            Atom::Prec => panic!("cannot peek self"),
            Atom::Parser { name } => quote!(input.peek::<#name>()),
            Atom::Token(token) => quote!(input.peek::<#token>()),
            Atom::Rule(rules) => {
                let iter = rules.iter().map(|item| item.peek());
                quote!(
                    #(#iter)||*
                )
            }
        }
    }

    pub fn create_parse(&self, level: u8) -> TokenStream {
        match self {
            Atom::Prec => quote!(__expression(input, #level)),
            Atom::Parser { name } => quote!(input.parse::<#name>()),
            Atom::Token(token) => quote!(input.parse::<#token>()),
            Atom::Rule(rules) => {
                let iter = rules.iter().map(|item| {
                    let peek = item.peek();
                    let parse = item.build_parse(level);

                    quote!(
                        if #peek {
                            #parse
                        }
                    )
                });
                quote!(
                    #(#iter)else*
                )
            }
        }
    }
}

impl Parse for Atom {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let rule = if input.peek(Token![@]) {
            let _ = input.parse::<Token![@]>()?;
            Self::Prec
        } else if input.peek(Ident) {
            let name = input.parse::<Ident>()?;
            Self::Parser { name }
        } else if input.peek(syn::token::Paren) {
            let group = input.parse::<Group>()?;

            let rule =
                Punctuated::<Rule, Token![/]>::parse_separated_nonempty.parse2(group.stream())?; //Precedence::parse.parse2(group.stream())?;

            Self::Rule(rule.into_iter().collect())
        } else if input.peek(LitStr) {
            Self::Token(AtomToken {
                name: input.parse()?,
            })
        } else {
            return Err(input.error(format!("expected rule item: {:?}", input)));
        };

        Ok(rule)
    }
}

#[derive(Debug)]
pub struct AtomToken {
    pub name: LitStr,
}

impl ToTokens for AtomToken {
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
