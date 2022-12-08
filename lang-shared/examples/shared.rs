use lang_parsing::ParseState;
use lang_shared::{parsing::Literal, tokens::*};

fn main() {
    let input = "fn test() { \"mig\" 30303 }";

    let mut parse = ParseState::<Token>::new(input).expect("tokens");

    if parse.peek(Literal) {}

    let ident = parse.parse::<(Ident, Ident, Punct, Punct, Punct, (Literal, Literal))>();

    println!("tokens: {:#?}", ident);
}
