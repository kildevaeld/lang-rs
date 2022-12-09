use super::parse::AstType;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

fn create_type(kind: &AstType) -> TokenStream {
    let ast_kind = &kind.name;

    let variants = kind.variants.iter().map(|item| {
        let name = &item.name;

        let struct_name =
            format_ident!("{}", format!("{}_{}", item.name, ast_kind).to_class_case());

        quote!(#name(#struct_name))
    });

    quote!(

        pub enum #ast_kind {
            #(
                #variants
            ),*
        }



    )
}

pub fn create(types: Vec<AstType>) -> TokenStream {
    let types = types.iter().map(create_type);

    quote!(
        #(
            #types
        )*
    )
}
