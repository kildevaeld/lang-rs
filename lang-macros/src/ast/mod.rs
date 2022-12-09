mod codegen;
mod parse;

use inflector::Inflector;
use proc_macro2::{Group, Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::parse::{Parse, Parser};
use syn::punctuated::Punctuated;
use syn::Token;

#[derive(Debug)]

struct Typo {}

impl Parse for Typo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            let group = input.parse::<Group>()?;
        }
        todo!()
    }
}

impl ToTokens for Typo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        quote!().to_tokens(tokens)
    }
}

#[derive(Debug)]
pub struct AstItemField {
    name: Ident,
    kind: Typo,
}

impl Parse for AstItemField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _colon = input.parse::<Token![:]>()?;

        let kind = input.parse::<Typo>()?;

        Ok(AstItemField { name, kind })
    }
}

#[derive(Debug)]
pub struct AstItem {
    name: Ident,
    fields: Vec<AstItemField>,
}

impl Parse for AstItem {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let group = input.parse::<Group>()?;

        let field_parser = Punctuated::<AstItemField, Token![,]>::parse_terminated;

        let fields = field_parser.parse2(group.stream())?;

        Ok(AstItem {
            name,
            fields: fields.into_iter().collect(),
        })
    }
}

pub struct AstType {
    name: Ident,
    items: Vec<AstItem>,
}

impl Parse for AstType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let group = input.parse::<Group>()?;
        let parser = Punctuated::<AstItem, Token![,]>::parse_terminated;

        let items = parser.parse2(group.stream())?;

        Ok(AstType {
            name,
            items: items.into_iter().collect(),
        })
    }
}

fn create_type(kind: AstType) -> impl ToTokens {
    let ast_kind = &kind.name;

    let structs = kind.items.iter().map(|item| {
        let name = &item.name;

        let struct_name = Ident::new(
            &format!("{}_{}", name, ast_kind).to_class_case(),
            Span::call_site(),
        );

        let fields = item
            .fields
            .iter()
            .map(|field| {
                //
                let name = &field.name;
                let kind = &field.kind;
                quote!(
                    #name: #kind
                )
            })
            .collect::<Vec<_>>();

        let field_names = item
            .fields
            .iter()
            .map(|field| {
                let name = &field.name;
                quote!(
                    #name
                )
            })
            .collect::<Vec<_>>();

        quote!(

            pub struct #struct_name {
                pub span: Span,
                #(pub #fields),*
            }

            impl #struct_name {
                pub fn new(#(#fields),*, span: Span) -> #struct_name {
                    #struct_name {
                        span,
                        #(#field_names),*

                    }
                }
            }

            impl From<#struct_name> for #ast_kind {
                fn from(ast: #struct_name) -> #ast_kind {
                    #ast_kind::#name(ast)
                }
            }

        )
    });

    let variants = kind.items.iter().map(|item| {
        //
        let variant_name = &item.name;

        let struct_name = Ident::new(
            &format!("{}_{}", variant_name, ast_kind).to_class_case(),
            Span::call_site(),
        );

        quote!(
            #variant_name(#struct_name)
        )
    });

    quote!(

        pub enum #ast_kind {
            #(#variants),*
        }

        #(
            #structs
        )*
    )
}

pub fn run(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    /*let parser = Punctuated::<AstType, Token![,]>::parse_terminated;

    let items = match parser.parse(input) {
        Ok(ret) => ret.into_iter().map(|item| create_type(item)),
        Err(err) => return err.into_compile_error().into(),
    };

    quote!(
        #(#items)*
    )
    .into()*/
    // println!("{:#?}", input);

    let types = parse::parse(input.into());

    let out = codegen::create(types);

    quote!().into()
}
