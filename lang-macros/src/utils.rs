use proc_macro2::{Ident, Span};
use proc_macro_crate::{crate_name, FoundCrate};

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
