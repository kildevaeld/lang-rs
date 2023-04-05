use proc_macro2::TokenStream;
use quote::quote;

mod codegen;
mod parser;

pub fn run(tokens: TokenStream) -> TokenStream {
    let bundle = parser::parse(tokens).expect("parse");

    codegen::run(bundle)
}
