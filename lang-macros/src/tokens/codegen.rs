use crate::utils::lang_parsing;
use proc_macro2::{Ident, Punct, Spacing, TokenStream, Span};
use quote::quote;

use super::parser::{Tokens, TypeList, Pair};

fn extract(tokens: &Tokens) -> TokenStream {
    let literals = tokens
        .literals
        .clone()
        .unwrap_or_else(|| TypeList {
            types: Vec::default(),
        })
        .types
        .into_iter()
        .map(|ty| quote!(#ty));

    quote!(
        (#(#literals),*, Punct<'input>, Ident<'input>)
    )
}

fn create_tokens(crate_name: &Ident, input: &Tokens) -> TokenStream {
    let keywords = input.keywords.pairs.iter().map(|item| {
        let token = &item.token;
        (
            &item.name,
            quote!(#crate_name::parsing::keyword_peek(cursor, #token)),
            quote!(#crate_name::parsing::keyword(input, #token)?.span),
            quote!(#crate_name::lexing::tokens::Ident<'input>),
        )
    });

    let puncts = input.puncts.pairs.iter().map(|item| {
        let token = &item.token;
        (
            &item.name,
            quote!(#crate_name::parsing::punctuation_peek(cursor, #token)),
            quote!(#crate_name::parsing::punctuation(input, #token)?),
            quote!(#crate_name::lexing::tokens::Punct<'input>),
        )
    });

    let items = keywords.chain(puncts).map(|(name, peek, parse, constraint)| {
        
        quote!(
            #[derive(Debug, Clone, Copy)]
            pub struct #name {
                pub span: #crate_name::lexing::Span
            }

            #[allow(non_snake_case)]
            pub fn #name(_span: #crate_name::lexing::Span) -> #name {
                panic!()
            }

            impl<'input, T> #crate_name::parsing::Token<'input, T> for #name
            where
                T: #crate_name::lexing::TokenRef<#constraint>,
                T: #crate_name::lexing::WithSpan,
            {

                fn peek(cursor: &mut #crate_name::parsing::Cursor<'input, '_, T>) -> bool {
                    #peek
                }
            }

            impl<'input, T> #crate_name::parsing::Parse<'input, T> for #name 
            where
                T: #crate_name::lexing::TokenRef<#constraint>,
                T: #crate_name::lexing::WithSpan,
            {
                fn parse(input: &mut #crate_name::parsing::TokenReader<'input, '_, T>) -> Result<Self, #crate_name::parsing::Error> {
                    let span = #parse;
                        Ok(#name {
                            span
                        })
                }
            }
        )
    });

    let module_path = input
        .module_path
        .as_ref()
        .map(|path| quote!($crate::#path))
        .unwrap_or_else(|| quote!($crate));

    let tokens = input.puncts.pairs.iter().map(|pair| {
        //
        let name = &pair.name;
        let token = &pair.token;

        let lit = token.token().to_string();

        let lit = &lit[1..lit.len() - 1];

        //println!("lit {}", pair.token.token().to_string());

        let token = match lit {
            "(" | ")" | "[" | "]" | "{" | "}" => {
                quote::quote!(#token)
            }
            _ => {
                let char_count = lit.chars().count();
                let punct = lit.chars().enumerate().map(|(i, c)| {
                    let spacing = if (i + 1) == char_count {
                        Spacing::Alone
                    } else {
                        Spacing::Joint
                    };
                    Punct::new(c, spacing)
                });
                quote::quote!(#(#punct)*)
            }
        };

        quote!([#token] =>{ #module_path::#name};)
    }).chain(input.keywords.pairs.iter().map(|item| {
        //
        let Pair {
            name,
            token
        } = item;

        let lit = token.token().to_string();
        let lit = &lit[1..lit.len() - 1];

        let token = Ident::new(lit, Span::call_site());
        
        quote!([#token] => { #module_path::#name };)
        
    }));


    quote!(

        #[macro_export]
        macro_rules! Token {
            #(#tokens)*
        }


        #(
            #items
        )*
    )
}

pub fn create(tokens: Tokens) -> TokenStream {
    let crate_name = lang_parsing();

    let extracts = extract(&tokens);

    let tokens = create_tokens(&crate_name, &tokens);

    quote!(

        pub type Extract<'input> = #extracts;

        pub type Lexer<'input> = #crate_name::lexing::Lexer<'input, Extract<'input>, #crate_name::lexing::tokens::Token<'input>>;

        #tokens
    )
}
