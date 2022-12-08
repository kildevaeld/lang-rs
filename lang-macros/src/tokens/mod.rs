mod codegen;
mod parser;

use proc_macro2::TokenStream;
use syn::parse::Parser;

pub fn run(input: TokenStream) -> TokenStream {
    let tokens = parser::parse.parse2(input).expect("parse input");

    codegen::create(tokens)
}
