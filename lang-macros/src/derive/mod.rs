mod parse;
mod peek;
mod with_span;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub fn run(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    parse::create(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub fn run_peek(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    peek::create(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

pub fn run_span(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    with_span::create(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
