use proc_macro2::TokenStream;

mod codegen;
mod parser;

pub fn run(tokens: TokenStream) -> TokenStream {
    let bundle = parser::parse(tokens).expect("parse");

    codegen::run(bundle)
}
