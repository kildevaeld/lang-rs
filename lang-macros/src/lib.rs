// mod ast;
mod derive;
#[cfg(feature = "opcodes")]
mod opcodes;
mod pratt;
mod tokens;
mod utils;
mod visitor;

mod pratt2;
mod pratt3;

use proc_macro::TokenStream;

/// tokens
#[proc_macro]
pub fn tokens(input: TokenStream) -> TokenStream {
    tokens::run(input.into()).into()
}

#[proc_macro]
pub fn precedence(input: TokenStream) -> TokenStream {
    pratt::run(input.into()).into()
}

#[proc_macro]
pub fn precedence2(input: TokenStream) -> TokenStream {
    pratt2::run(input.into()).into()
}

#[proc_macro]
pub fn precedence3(input: TokenStream) -> TokenStream {
    pratt3::run(input.into()).into()
}

#[cfg(feature = "opcodes")]
#[proc_macro]
pub fn opcodes(input: TokenStream) -> TokenStream {
    opcodes::run(input)
}

#[proc_macro_derive(Parse, attributes(parse))]
pub fn parse_derive(item: TokenStream) -> TokenStream {
    derive::run(item)
}

#[proc_macro_derive(Peek)]
pub fn peek_derive(item: TokenStream) -> TokenStream {
    derive::run_peek(item)
}

#[proc_macro_derive(WithSpan)]
pub fn with_span_derive(item: TokenStream) -> TokenStream {
    derive::run_span(item)
}

#[proc_macro_attribute]
pub fn visitor(attr: TokenStream, item: TokenStream) -> TokenStream {
    visitor::run(attr, item)
}
