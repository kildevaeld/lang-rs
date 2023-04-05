use proc_macro2::TokenStream;
use quote::quote;

mod parser;

pub fn run(tokens: TokenStream) -> TokenStream {
    let bundle = parser::parse(tokens).expect("parse");

    println!("{:#?}", bundle);

    // if bundle.rule_list.is_empty() {
    //     panic!("no items");
    // }

    // codegen::run(bundle)

    quote!()
}
