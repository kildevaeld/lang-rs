use crate::utils::lang_parsing;

use super::parse::{RuleItem, RuleList, Token as ParseToken};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{token::Type, Block};

impl ParseToken {
    fn peek(&self) -> TokenStream {
        match self {
            ParseToken::Any(any) => {
                let i = any.iter().map(|i| {
                    //

                    quote!(input.peek(#i))
                });

                quote!(
                    #(#i)||*
                )
            }
            ParseToken::Operator(lit) => {
                quote!(input.peek(#lit))
            }
            ParseToken::Ident(ident) => {
                quote!(input.clone().parse::<#ident>().is_ok())
            }
        }
    }
}

pub struct Input {
    pub module: Ident,
    pub return_type: Type,
    pub primary: Option<Block>,
    pub rules: Vec<RuleList>,
}

fn build_precedence_list(rules: &[RuleList]) -> TokenStream {
    let list = rules
        .iter()
        .enumerate()
        .map(|(idx, rules)| {
            rules
                .rules
                .iter()
                .filter(|item| item.is_infix())
                .filter_map(move |item| {
                    let operator = &item.items[1];
                    let item = match operator {
                        RuleItem::Token(token) => token.peek(),
                        RuleItem::Parse { parse, .. } => parse.peek(),
                        _ => return None,
                    };

                    let prec = idx as u8 + 1;

                    Some(quote!(
                        if #item {
                            #prec
                        }
                    ))
                })
        })
        .flatten();

    quote!(
        #(#list)else*
        else {
            0u8
        }
    )
}

fn peek_token(token: &ParseToken) -> TokenStream {
    let tokens = match token {
        ParseToken::Operator(lit) => {
            vec![quote!(#lit)]
        }

        ParseToken::Any(any) => any.into_iter().map(|m| quote!(#m)).collect(),
        ParseToken::Ident(ident) => return quote!(input.clone().parse::<#ident>().is_ok()),
    };

    quote!(
        #(
            input.peek(#tokens)
        )||*
    )
}

fn parse_token(name: impl Into<Option<Ident>>, token: &ParseToken) -> TokenStream {
    let name = name.into().unwrap_or_else(|| format_ident!("_"));

    let (tokens, error_msg) = match token {
        ParseToken::Operator(lit) => (vec![quote!(#lit)], format!("expected: {}", lit)),

        ParseToken::Any(any) => (
            any.into_iter().map(|m| quote!(#m)).collect(),
            format!(
                "expected any of: {}",
                any.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        ),
        ParseToken::Ident(ident) => return quote!(let #name = input.parse::<#ident>()?;),
    };

    quote!(
        let #name = #(
            if let Ok(ret) = input.parse::<#tokens>() {
                ret.span
            }
        )else*
        else {
            return Err(input.error(#error_msg))
        };
    )
}

fn build_infix(rules: &[RuleList]) -> TokenStream {
    let list = rules
        .iter()
        .enumerate()
        .map(|(idx, rules)| {
            rules
                .rules
                .iter()
                .filter(|item| item.is_infix())
                .filter_map(move |item| {
                    let action = &item.action;

                    let prec = idx as u8 + 1;

                    let first = item
                        .items
                        .get(0)
                        .map(|item| match item {
                            RuleItem::Parse { name, .. } => {
                                quote!(let #name = left;)
                            }
                            RuleItem::Prec { name } => {
                                quote!(let #name = left;)
                            }
                            RuleItem::Token(_) => {
                                panic!("token not supported at the place")
                            }
                        })
                        .expect("first");

                    let peek = item
                        .items
                        .get(1)
                        .map(|item| match item {
                            RuleItem::Prec { .. } => {
                                quote!(__expression(&mut input.clone(), 0).is_ok())
                            }
                            RuleItem::Parse { parse, .. } => peek_token(parse),
                            RuleItem::Token(token) => peek_token(token),
                        })
                        .unwrap();

                    let types = item.items.iter().skip(1).map(|item| match item {
                        RuleItem::Prec { name } => {
                            quote!(let #name = __expression(input, #prec)?;)
                        }
                        RuleItem::Parse { name, parse } => {
                            //
                            parse_token(name.clone(), parse)
                        }
                        RuleItem::Token(token) => {
                            //
                            parse_token(None, token)
                        }
                    });

                    Some(quote!(
                        if #peek {
                            #first
                            #(
                             #types
                            )*
                            #action
                         }
                    ))
                })
        })
        .flatten();

    quote!(
        #(#list)else*
        else {
            panic!("error")
        }
    )
}

fn build_prefix(rules: &[RuleList]) -> TokenStream {
    let list = rules
        .iter()
        .map(|rules| {
            rules
                .rules
                .iter()
                .rev()
                .filter(|item| !item.is_infix())
                .map(move |item| {
                    let names = item.items.iter().map(|item| {
                        //
                        match item {
                            RuleItem::Parse { name, parse } => parse_token(name.clone(), parse),
                            RuleItem::Prec { name } => {
                                quote!(let #name = __expression(input, 0)?;)
                            }
                            RuleItem::Token(token) => parse_token(None, token),
                        }
                    });

                    let types = item
                        .items
                        .first()
                        .map(|item| {
                            //
                            match item {
                                RuleItem::Parse { parse, .. } => peek_token(parse),
                                RuleItem::Prec { .. } => {
                                    quote!(__expression(&mut input.clone(), 0).is_ok())
                                }
                                RuleItem::Token(token) => peek_token(token),
                            }
                        })
                        .unwrap();

                    let action = &item.action;

                    let items = quote!(
                        if #types {
                            #(
                                #names
                            )*
                            #action
                        }

                    );

                    //
                    items
                })
        })
        .flatten();

    quote!(
        if let Result::<_, Error>::Ok(ret) = __primary(input) {
            return Ok(ret)
        } #(#list)else*
        else {
            Err(input.error("invalid prefix expression"))
        }

    )
}

pub fn run(input: Input) -> TokenStream {
    let Input {
        module,
        return_type,
        primary,
        rules,
    } = input;

    let crate_name = lang_parsing();

    let precedence_rules = build_precedence_list(&rules);

    let infix = build_infix(&rules);

    let prefix = build_prefix(&rules);

    let primary = primary
        .map(|b| quote!(#b))
        .unwrap_or_else(|| quote!({ input.error("no primary") }));

    quote!(

        #[allow(unused_braces, non_snake_case)]
        pub mod #module {
            use super::*;
            use #crate_name::{parsing::{Parse, TokenReader, Error}, lexing::tokens::Token};

            fn __primary<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> #primary


            fn __get_precedence<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> u8 {
                #precedence_rules
            }

            fn __prefix<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                #prefix
            }

            fn __infix<'input>(input: &mut TokenReader<'input, '_, Token<'input>>, left: #return_type) -> Result<#return_type, Error> {
                 #infix
            }

            fn __expression<'input>(input: &mut TokenReader<'input, '_, Token<'input>>, precedence: u8) -> Result<#return_type, Error> {
                let mut left = __prefix(input)?;

                while precedence < __get_precedence(input) {
                    left = __infix(input, left)?;
                }

                Ok(left)
            }

            pub fn parser<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                __expression(input, 0)
            }

            impl<'input> Parse<'input, Token<'input>> for #return_type {
                fn parse(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                    parser(input)
                }
            }

        }

    )
}
