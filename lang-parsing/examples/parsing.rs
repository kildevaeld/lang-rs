use lang_lexing::{tokens::*, LexerFactory};
use lang_parsing::{tokens::Ws, Parser};

fn main() {
    let input = "fn test() { \"mig\" 30303 }";

    let lexer = Token::create_lexer(input).skip_whitespace(false);

    let mut parse = Parser::<Token>::from_tokens(input, lexer.tokenize()).expect("parser");

    if parse.peek::<Literal>() {}

    let ident = parse.parse::<(
        Ws<Ident>,
        Ws<Ident>,
        Ws<Punct>,
        Ws<Punct>,
        Ws<Punct>,
        (Ws<Literal>, Ws<Literal>),
    )>();

    println!("tokens: {ident:#?}");
}
