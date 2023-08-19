use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics};

use super::peek::fields_peek;
use crate::utils::{fields_is_tuple, generics_impl, lang_parsing};

pub fn create(input: DeriveInput) -> syn::Result<TokenStream> {
    match input.data {
        Data::Enum(e) => create_enum(input.ident, input.generics, e),
        Data::Struct(s) => create_struct(input.ident, input.generics, s),
        _ => Err(syn::Error::new(Span::call_site(), "unsupported type")),
    }
}

fn fields_parse(fields: &Fields) -> syn::Result<Vec<TokenStream>> {
    let fields = fields
        .iter()
        .map(|field| {
            let name = &field.ident;

            let ret = match name {
                Some(name) => quote!(
                    #name: state.parse()?
                ),
                None => quote!(state.parse()?),
            };

            syn::Result::Ok(ret)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(fields)
}

fn create_struct(name: Ident, generics: Generics, data: DataStruct) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();
    let tuple = data.fields.iter().any(|m| m.ident.is_none());

    let fields = fields_parse(&data.fields)?;

    let (generics_impl, _, ty_gen, where_clause) = generics_impl(&generics);

    let instance = if tuple {
        quote!(#name(#(#fields),*))
    } else {
        quote!(#name { #(#fields),* })
    };

    let out = quote!(
        impl #generics_impl #crate_name::parsing::Parse<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn parse(mut state: #crate_name::parsing::TokenReader<'parse, '_, #crate_name::lexing::tokens::Token<'parse>>) -> Result<Self, #crate_name::parsing::Error> {
                Ok(#instance)
            }
        }
    );

    Ok(out)
}

fn create_enum(name: Ident, generics: Generics, data: DataEnum) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let parse = data
        .variants
        .iter()
        .map(|variant| {
            if variant.fields.is_empty() {
                return Err(syn::Error::new(variant.ident.span(), "no fields"));
            }

            let variant_name = &variant.ident;

            let fields = fields_parse(&variant.fields)?;

            let instance = if fields_is_tuple(&variant.fields) {
                quote!(#name::#variant_name(#(#fields),*))
            } else {
                quote!(#name::#variant_name { #(#fields),* })
            };

            let peek = fields_peek(&variant.fields, false)?;

            let instance = quote!(
                if #peek {
                    return Ok(#instance)
                }
            );

            Ok(instance)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (generics_impl, _, ty_gen, where_clause) = generics_impl(&generics);

    let error = data.variants.iter().map(|m| {
        let name = m.ident.to_string();
        quote!(
            #crate_name::parsing::ErrorKind::Expected {
                message: #name.into(),
                rule: Some(#name.into())
            }
        )
    });

    let name_str = name.to_string();

    let ret = quote!(
        impl #generics_impl #crate_name::parsing::Parse<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn parse(mut state: #crate_name::parsing::TokenReader<'parse, '_, #crate_name::lexing::tokens::Token<'parse>>) -> Result<Self, #crate_name::parsing::Error> {
                use #crate_name::lexing::WithSpan;

                let span = state.current().map(|m| m.span()).unwrap_or_default();


                #(#parse)else*

                let errors = vec![#(#error),*];

                Err(#crate_name::parsing::Error::new((#name_str, errors), span))
            }
        }



    );

    Ok(ret)
}
