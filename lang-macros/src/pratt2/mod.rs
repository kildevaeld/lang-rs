mod codegen;
mod parser;

use proc_macro2::TokenStream;
use quote::quote;

pub fn run(tokens: TokenStream) -> TokenStream {
    let bundle = parser::parse(tokens).expect("parse");

    if bundle.rule_list.is_empty() {
        panic!("no items");
    }

    codegen::run(bundle)
}
