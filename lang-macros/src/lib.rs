// mod ast;
mod derive;
#[cfg(feature = "opcodes")]
mod opcodes;
mod pratt;
mod tokens;
mod utils;

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

#[cfg(feature = "opcodes")]
#[proc_macro]
pub fn opcodes(input: TokenStream) -> TokenStream {
    opcodes::run(input)
}

#[proc_macro_derive(Parse, attributes(parse))]
pub fn tlborm_derive(item: TokenStream) -> TokenStream {
    derive::run(item)
}

// #[proc_macro]
// pub fn ast(input: TokenStream) -> TokenStream {
//     ast::run(input)
// }
