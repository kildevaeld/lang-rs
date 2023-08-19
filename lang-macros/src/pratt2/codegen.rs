use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::lang_parsing;

use super::parser::{Pratt, RuleGroup};

pub fn run(bundle: Pratt) -> TokenStream {
    let crate_name = lang_parsing();

    let module_name = bundle.module_name;
    let return_type = bundle.return_type;

    let peek = create_peek(&bundle.rules, true, true);
    let precedence = build_precedence_list(&bundle.rules);
    let prefix = build_rule_list(&bundle.rules, true);
    let infix = build_rule_list(&bundle.rules, false);

    quote!(

        #[allow(unused_braces, non_snake_case)]
        mod #module_name {
            use super::*;
            use #crate_name::{parsing::{Parse, Peek, TokenReader, Error}, lexing::{tokens::Token, WithSpan, Span}};

            fn __peek<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> bool {
                #peek
            }


            fn __get_precedence<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> u8 {
                #precedence
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

            pub fn parse<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                __expression(input, 0)
            }

            impl<'input> Parse<'input, Token<'input>> for #return_type {
                fn parse(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                    parse(input)
                }
            }

            impl<'input> Peek<'input, Token<'input>> for #return_type {
                fn peek(cursor: &mut TokenReader<'input, '_, Token<'input>>) -> bool {
                    __peek(cursor)
                }
            }


        }

    )
}

fn create_peek(groups: &[RuleGroup], prefix: bool, for_peek: bool) -> TokenStream {
    let iter = groups
        .iter()
        .filter_map(|group| create_peek_for_group(group, prefix, for_peek))
        .collect::<Vec<_>>();

    quote!(
        #(#iter)||*
    )
}

fn create_peek_for_group(group: &RuleGroup, prefix: bool, for_peek: bool) -> Option<TokenStream> {
    let iter = group
        .rules
        .iter()
        .flat_map(|m| {
            m.rules.iter().filter_map(|m| {
                if (prefix && m.is_prefix()) || (!prefix && !m.is_prefix()) {
                    if for_peek {
                        Some(m.peek_prefix())
                    } else {
                        Some(m.peek())
                    }
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    if iter.is_empty() {
        None
    } else {
        Some(quote!(
            #(#iter)||*
        ))
    }
}

fn build_precedence_list(rules: &[RuleGroup]) -> TokenStream {
    let list = rules.iter().enumerate().filter_map(|(idx, group)| {
        let level = create_peek_for_group(group, false, false)?;

        let prec = idx as u8 + 1;

        Some(quote!(
            if #level {
                #prec
            }
        ))
    });

    quote!(
        #(#list)else*
        else {
            0u8
        }
    )
}

fn build_rule_list(rules: &[RuleGroup], prefix: bool) -> TokenStream {
    let output = rules
        .iter()
        .enumerate()
        .filter_map(|(idx, group)| build_rule(group, prefix, (idx + 1) as u8))
        .collect::<Vec<_>>();

    let error = if prefix { "prefix" } else { "infix" };

    if output.is_empty() {
        quote!(Err(input.error("no prefix")))
    } else {
        quote!(
            #(#output)else*
            else {
                return Err(input.error(#error))
            }
        )
    }
}

fn build_rule(group: &RuleGroup, prefix: bool, level: u8) -> Option<TokenStream> {
    let output = group
        .rules
        .iter()
        .filter_map(|m| {
            let rules = m
                .rules
                .iter()
                .filter_map(|m| {
                    if (prefix && m.is_prefix()) || (!prefix && !m.is_prefix()) {
                        Some(m.build_parse(level))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if rules.is_empty() {
                None
            } else {
                Some(quote!(
                    #(#rules)else*
                ))
            }
        })
        .collect::<Vec<_>>();

    if output.is_empty() {
        None
    } else {
        Some(quote!(
            #(#output)else*

        ))
    }
}
