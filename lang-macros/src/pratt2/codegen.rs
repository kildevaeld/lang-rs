use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::utils::lang_parsing;

use super::parser::{Bundle, Rule, RuleEntry, RuleGroup};

pub fn run(bundle: Bundle) -> TokenStream {
    let crate_name = lang_parsing();

    let module_name = bundle.module;
    let return_type = bundle.return_type;

    let precedence_list = build_precedence_list(&bundle.rule_list);
    let prefix_fn = build_prefix(&bundle.rule_list);
    let infix_fn = build_infix(&bundle.rule_list);

    quote!(

        #[allow(unused_braces, non_snake_case)]
        mod #module_name {
            use super::*;
            use #crate_name::{parsing::{Parse, Peek, TokenReader, Cursor, Error}, lexing::{tokens::Token, WithSpan, Span}};

            fn __get_precedence<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> u8 {
                #precedence_list
            }

            fn __prefix<'input>(input: &mut TokenReader<'input, '_, Token<'input>>) -> Result<#return_type, Error> {
                #prefix_fn

            }

            fn __infix<'input>(input: &mut TokenReader<'input, '_, Token<'input>>, left: #return_type) -> Result<#return_type, Error> {
                 #infix_fn

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


        }

    )
}

fn build_precedence_list(rules: &[RuleGroup]) -> TokenStream {
    let list = rules.iter().enumerate().flat_map(|(idx, rules)| {
        let level = rules
            .rules
            .iter()
            .filter(|item| !item.is_prefix())
            .filter_map(move |item| item.peek());

        let prec = idx as u8 + 1;

        Some(quote!(
            if #(#level)||* {
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

impl Rule {
    pub fn parse_token(&self, prefix: bool) -> TokenStream {
        let peek = self.peek().expect("peek");

        let mut names = vec![];
        let mut values = vec![];
        let skip = if prefix { 0 } else { 1 };
        self.items.iter().skip(skip).for_each(|entry| match entry {
            RuleEntry::Alternatives(s) => {
                let iter = s.iter().map(|m| {
                    let peek = m.peek();
                    let name = m
                        .name
                        .as_ref()
                        .map(|m| quote!(#m))
                        .unwrap_or_else(|| quote!(_));
                    let parse = m.kind.to_parse(prefix);
                    quote!(
                        if #peek {
                            let #name = #parse?;
                        }
                    )
                });

                names.push(quote!(_));
                values.push(quote!(
                    #(#iter)else*
                    else {
                        panic!("alternative")
                    }
                ));
            }
            RuleEntry::Single(s) => {
                let name = s
                    .name
                    .as_ref()
                    .map(|m| quote!(#m))
                    .unwrap_or_else(|| quote!(_));
                names.push(name);
                values.push(s.kind.to_parse(prefix));
            }
        });

        let map = self
            .map
            .as_ref()
            .map(|m| quote!(#m))
            .unwrap_or_else(|| quote!({}));

        quote!(
            if #peek {
                let (#(#names),*) = (#(#values),*);
                #map
            }
        )
    }
}

fn build_prefix(rules: &[RuleGroup]) -> TokenStream {
    let rules = rules.iter().flat_map(|rule| {
        rule.rules
            .iter()
            .filter(|rule| rule.is_prefix())
            .map(|rule| rule.parse_token(true))
    });

    quote!(
        #(#rules)else*
        else {
            panic!("")
        }
    )
}

fn build_infix(rules: &[RuleGroup]) -> TokenStream {
    let rules = rules.iter().flat_map(|rule| {
        rule.rules
            .iter()
            .filter(|rule| !rule.is_prefix())
            .map(|rule| rule.parse_token(false))
    });

    quote!(
        #(#rules)else*
        else {
            panic!("")
        }
    )
}
