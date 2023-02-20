use crate::utils::{generics_impl, lang_parsing, path_type, type_is_option};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Fields, Generics};

pub fn create(input: DeriveInput) -> syn::Result<TokenStream> {
    match input.data {
        Data::Enum(e) => create_enum(input.ident, input.generics, e),
        Data::Struct(s) => create_struct(input.ident, input.generics, s),
        _ => Err(syn::Error::new(Span::call_site(), "unsupported type")),
    }
}

pub fn fields_peek(fields: &Fields, cursor: bool) -> syn::Result<TokenStream> {
    let fields = fields.iter().collect::<Vec<_>>();
    let ident = {
        if type_is_option(&fields.first().expect("first").ty) {
            todo!("first field cannot be an option")
        }

        let ident = path_type(&fields.first().unwrap().ty);
        ident
    };

    let peek = if cursor {
        quote!(<#ident as lang::parsing::Peek<'parse, lang::lexing::tokens::Token<'parse>>>::peek(cursor))
    } else {
        quote!(state.peek::<#ident>())
    };
    Ok(peek)
}

fn create_enum(name: Ident, generics: Generics, data: DataEnum) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let peeks = data
        .variants
        .iter()
        .map(|variant| fields_peek(&variant.fields, true))
        .collect::<Result<Vec<_>, _>>()?;

    let (generics_impl, _, ty_gen, where_clause) = generics_impl(&generics);

    let token_stream = quote!(

        impl #generics_impl #crate_name::parsing::Peek<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn peek(cursor: &mut #crate_name::parsing::Cursor<'parse, '_,  #crate_name::lexing::tokens::Token<'parse>>) -> bool {
                use #crate_name::parsing::Peek;
                #(#peeks)||*
            }
        }

    );

    Ok(token_stream)
}

fn create_struct(name: Ident, generics: Generics, data: DataStruct) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let peek = fields_peek(&data.fields, true)?;

    let (generics_impl, _, ty_gen, where_clause) = generics_impl(&generics);

    Ok(quote!(

        impl #generics_impl #crate_name::parsing::Peek<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn peek(cursor: &mut #crate_name::parsing::Cursor<'parse, '_,  #crate_name::lexing::tokens::Token<'parse>>) -> bool {
                use #crate_name::parsing::Peek;
                #peek
            }
        }

    ))
}
