mod pratt;
mod tokens;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn tokens(input: TokenStream) -> TokenStream {
    tokens::run(input.into()).into()
}

#[proc_macro]
pub fn precedence(input: TokenStream) -> TokenStream {
    pratt::run(input.into()).into()
}
