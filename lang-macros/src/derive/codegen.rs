use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Generics, ImplGenerics, Path, Type,
    TypeGenerics, WhereClause,
};

use crate::utils::lang_parsing;

pub fn create(input: DeriveInput) -> syn::Result<TokenStream> {
    match input.data {
        Data::Enum(e) => create_enum(input.ident, input.generics, e),
        Data::Struct(s) => create_struct(input.ident, input.generics, s),
        _ => Err(syn::Error::new(Span::call_site(), "unsupported type")),
    }
}

fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}

fn path_ident(path: &Path) -> Option<TokenStream> {
    if path.segments.is_empty() {
        None
    } else {
        let segments = path.segments.iter().map(|m| &m.ident);
        Some(quote!(#(#segments)::*))
    }
}

fn path_type(ty: &Type) -> Option<TokenStream> {
    match ty {
        Type::Path(p) => path_ident(&p.path),
        Type::Macro(m) => Some(quote!(#m)),
        _ => None,
    }
}

fn type_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(p) => path_is_option(&p.path),
        _ => false,
    }
}

fn fields_is_tuple(fields: &Fields) -> bool {
    fields.iter().any(|m| m.ident.is_none())
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

fn fields_peek(fields: &Fields, cursor: bool) -> syn::Result<TokenStream> {
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

fn generics_impl(
    generics: &Generics,
) -> (
    TokenStream,
    ImplGenerics,
    TypeGenerics<'_>,
    Option<&'_ WhereClause>,
) {
    let (origin_generics_impl, ty_gen, where_clause) = generics.split_for_impl();

    let lifetimes = generics.lifetimes().collect::<Vec<_>>();
    let params = &generics.params;

    let generics_impl = if lifetimes.is_empty() {
        if params.is_empty() {
            quote!(<'parse>)
        } else {
            quote!(<'parse, #params>)
        }
    } else {
        let constrain = quote!(#(#lifetimes)+*);
        quote!(<'parse: #constrain, #params>)
    };

    (generics_impl, origin_generics_impl, ty_gen, where_clause)
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

fn create_struct(name: Ident, generics: Generics, data: DataStruct) -> syn::Result<TokenStream> {
    let crate_name = lang_parsing();
    let tuple = data.fields.iter().any(|m| m.ident.is_none());

    let fields = fields_parse(&data.fields)?;

    let (generics_impl, original_generics_impl, ty_gen, where_clause) = generics_impl(&generics);

    let instance = if tuple {
        quote!(#name(#(#fields),*))
    } else {
        quote!(#name { #(#fields),* })
    };

    let fields = data.fields.iter().collect::<Vec<_>>();

    let peek = fields_peek(&data.fields, true)?;

    let span = fields_span(Span::call_site(), &fields)?;

    let out = quote!(
        impl #generics_impl #crate_name::parsing::Parse<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn parse(state: &mut  #crate_name::parsing::TokenReader<'parse, '_, #crate_name::lexing::tokens::Token<'parse>>) -> Result<Self, #crate_name::parsing::Error> {
                Ok(#instance)
            }
        }

        impl #generics_impl #crate_name::parsing::Peek<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn peek(cursor: &mut #crate_name::parsing::Cursor<'parse, '_,  #crate_name::lexing::tokens::Token<'parse>>) -> bool {
                #peek
            }
        }

        impl #original_generics_impl #crate_name::lexing::WithSpan for #name #ty_gen #where_clause {
            fn span(&self) -> #crate_name::lexing::Span {
                #span
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

    let peeks = data
        .variants
        .iter()
        .map(|variant| fields_peek(&variant.fields, true))
        .collect::<Result<Vec<_>, _>>()?;

    let (generics_impl, original_generics_impl, ty_gen, where_clause) = generics_impl(&generics);

    let error = data
        .variants
        .iter()
        .map(|m| {
            // println!("attr: {:#?}", m.attrs);
            m.ident.to_string()
        })
        .collect::<Vec<_>>()
        .join(" or ");

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

    let ret = quote!(
        impl #generics_impl #crate_name::parsing::Parse<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn parse(state: &mut  #crate_name::parsing::TokenReader<'parse, '_, #crate_name::lexing::tokens::Token<'parse>>) -> Result<Self, #crate_name::parsing::Error> {
                #(#parse)else*
                Err(state.error(#error))
            }
        }

        impl #generics_impl #crate_name::parsing::Peek<'parse, #crate_name::lexing::tokens::Token<'parse>> for #name #ty_gen #where_clause {
            fn peek(cursor: &mut #crate_name::parsing::Cursor<'parse, '_,  #crate_name::lexing::tokens::Token<'parse>>) -> bool {
                #(#peeks)||*
            }
        }

        impl #original_generics_impl #crate_name::lexing::WithSpan for #name #ty_gen #where_clause {
            fn span(&self) -> #crate_name::lexing::Span {
                match self {
                    #(#span),*
                }
            }
        }

    );

    Ok(ret)
}
