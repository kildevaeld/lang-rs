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

fn fields_span(span: Span, fields: &[&Field]) -> syn::Result<TokenStream> {
    let first = fields
        .first()
        .ok_or_else(|| syn::Error::new(span, "first field cannot be optional"))?;

    let last = fields
        .first()
        .ok_or_else(|| syn::Error::new(span, "last field cannot be optional"))?;

    let first_field = first
        .ident
        .as_ref()
        .map(|m| quote!(#m))
        .unwrap_or_else(|| quote!(0));

    // if fields.len() > 1 {
    //     let first = fields
    //         .first()
    //         .ok_or_else(|| syn::Error::new(span, "first field cannot be optional"))?;

    //     fields.iter().skip(1).fold(
    //         quote!(self.#first.span()),
    //         |prev, next| quote!(#prev + #next.span()),
    //     );
    // }

    let ret = if first == last {
        quote!(self.#first_field.span())
    } else {
        let last = last.ident.as_ref().map(|m| quote!(#m)).unwrap_or_else(|| {
            let len = fields.len();
            quote!(#len)
        });
        quote!(self.#first + self.#last)
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

    let span = fields_span(Span::call_site(), &fields)?;

    let (_, original_generics_impl, ty_gen, where_clause) = generics_impl(&generics);

    Ok(quote!(

        impl #original_generics_impl #crate_name::lexing::WithSpan for #name #ty_gen #where_clause {
            fn span(&self) -> #crate_name::lexing::Span {
                #span
            }
        }

    ))
}
