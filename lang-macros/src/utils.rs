use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{
    Fields, GenericArgument, Generics, ImplGenerics, Path, PathArguments, Type, TypeGenerics,
    TypePath, WhereClause,
};

pub fn lang_parsing() -> Ident {
    let found_crate = crate_name("lang").expect("lang is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => {
            if !cfg!(test) {
                Ident::new("lang", Span::call_site())
            } else {
                Ident::new("crate", Span::call_site())
            }
        }
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            ident
        }
    }
}

#[cfg(feature = "serde")]
pub fn serde_crate() -> Ident {
    let found_crate = crate_name("serde").expect("serde is present in `Cargo.toml`");

    match found_crate {
        FoundCrate::Itself => Ident::new("serde", Span::call_site()),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            ident
        }
    }
}

pub fn path_is_option(path: &Path) -> bool {
    path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments.iter().next().unwrap().ident == "Option"
}

pub fn type_is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(p) => path_is_option(&p.path),
        _ => false,
    }
}

pub fn fields_is_tuple(fields: &Fields) -> bool {
    fields.iter().any(|m| m.ident.is_none())
}

fn nullify_lifetimes(ty: &mut TypePath) {
    ty.path.segments.iter_mut().for_each(|segment| {
        if let PathArguments::AngleBracketed(bargs) = &mut segment.arguments {
            for arg in &mut bargs.args {
                match arg {
                    GenericArgument::Type(Type::Path(ty)) => nullify_lifetimes(ty),
                    GenericArgument::Lifetime(lifetime) => lifetime.ident = format_ident!("_"),
                    _ => return,
                }
            }
        }
    })
}

pub fn path_type(ty: &Type) -> Option<TokenStream> {
    match ty {
        Type::Path(p) => {
            let mut clone = p.clone();
            nullify_lifetimes(&mut clone);
            Some(quote!(#clone))
        }
        Type::Macro(m) => Some(quote!(#m)),
        _ => None,
    }
}

pub fn generics_impl(
    generics: &Generics,
) -> (
    TokenStream,
    ImplGenerics<'_>,
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
