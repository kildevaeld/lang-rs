use crate::utils::{fields_is_tuple, generics_impl, lang_parsing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Generics};

pub fn create(input: DeriveInput) -> syn::Result<TokenStream> {
    match input.data {
        Data::Enum(e) => create_enum(input.ident, input.generics, e),
        Data::Struct(s) => create_struct(input.ident, input.generics, s),
        _ => Err(syn::Error::new(Span::call_site(), "unsupported type")),
    }
}

fn fields_span(fields: &[&Field]) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let first = fields
        .first()
        .map(|first| {
            first
                .ident
                .as_ref()
                .map(|ident| quote!(self.#ident.span()))
                .unwrap_or_else(|| quote!(self.0.span()))
        })
        .unwrap_or_else(|| quote!(#crate_name::lexing::Span::default()));

    let ret = if fields.len() > 1 {
        fields
            .iter()
            .skip(1)
            .enumerate()
            .fold(first, |prev, (idx, next)| {
                let next = next
                    .ident
                    .as_ref()
                    .map(|next| quote!(self.#next.span()))
                    .unwrap_or_else(|| quote!(self.#idx.span()));
                quote!(#prev + #next)
            })
    } else {
        first
    };

    Ok(ret)
}

fn fields_names(fields: &Fields) -> syn::Result<Vec<TokenStream>> {
    let fields = fields
        .iter()
        .enumerate()
        .map(|(idx, field)| {
            //
            let name = &field.ident;

            let ret = match name {
                Some(name) => quote!(
                    #name
                ),
                None => {
                    let name = format_ident!("field_{}", idx);
                    quote!(#name)
                }
            };

            syn::Result::Ok(ret)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(fields)
}

fn create_enum(name: Ident, generics: Generics, data: DataEnum) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let span = data
        .variants
        .iter()
        .map(|variant| {
            //
            let fields = fields_names(&variant.fields)?;

            let first = fields
                .first()
                .ok_or_else(|| syn::Error::new(Span::call_site(), "expected field"))?;

            let last = fields
                .last()
                .ok_or_else(|| syn::Error::new(Span::call_site(), "expected field"))?;

            let span = if fields.len() == 1 {
                quote!(#first.span())
            } else {
                quote!(#first.span() + #last.span())
            };

            let variant_name = &variant.ident;

            let ret = if fields_is_tuple(&variant.fields) {
                quote!(
                    #name::#variant_name(#(#fields),*) => {
                        #span
                    }
                )
            } else {
                quote!(
                    #name::#variant_name {
                        #(#fields),*
                    } => {
                        #span
                    }
                )
            };

            syn::Result::Ok(ret)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (_, original_generics_impl, ty_gen, where_clause) = generics_impl(&generics);

    let token_stream = quote!(

        impl #original_generics_impl #crate_name::lexing::WithSpan for #name #ty_gen #where_clause {
            fn span(&self) -> #crate_name::lexing::Span {
                match self {
                    #(#span),*
                }
            }
        }

    );

    Ok(token_stream)
}

fn create_struct(name: Ident, generics: Generics, data: DataStruct) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();

    let fields = data.fields.iter().collect::<Vec<_>>();

    let span = fields_span(&fields)?;

    let (_, original_generics_impl, ty_gen, where_clause) = generics_impl(&generics);

    Ok(quote!(

        impl #original_generics_impl #crate_name::lexing::WithSpan for #name #ty_gen #where_clause {
            fn span(&self) -> #crate_name::lexing::Span {
                #span
            }
        }

    ))
}
