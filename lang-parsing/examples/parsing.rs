use lang_lexing::{tokens::*, LexerFactory};
use lang_parsing::{
    tokens::{NoWs, Ws},
    Parser,
};

fn main() {
    let input = "fn test() { \"mig\" 30303 test.mig }";

    let lexer = Token::create_lexer(input).skip_whitespace(false);

    let mut parse = Parser::<Token>::from_tokens(input, lexer.tokenize()).expect("parser");

    if parse.peek::<Literal>() {}

    let ident = parse.parse::<(
        Ident,
        Ident,
        Punct,
        Punct,
        Ws<Punct>,
        (Literal, Literal),
        Ident,
        NoWs<Punct>,
        Ident,
    )>();

    println!("tokens: {ident:#?}");
}
