mod codegen;
mod parser;

use proc_macro2::TokenStream;
use quote::quote;

pub fn run(tokens: TokenStream) -> TokenStream {
    let bundle = parser::parse(tokens).expect("parse");

    codegen::run(bundle)
}
