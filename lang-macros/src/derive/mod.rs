mod codegen;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub fn run(stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as DeriveInput);

    codegen::create(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
