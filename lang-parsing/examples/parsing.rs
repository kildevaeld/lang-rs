use lang_lexing::{tokens::*, LexerFactory};
use lang_parsing::Parser;

fn main() {
    let input = "fn test() { \"mig\" 30303 }";

    let lexer = Token::create_lexer(input).skip_whitespace(true);

    let mut parse = Parser::<Token>::from_tokens(input, lexer.tokenize()).expect("parser");

    if parse.peek::<Literal>() {}

    let ident = parse.parse::<(Ident, Ident, Punct, Punct, Punct, (Literal, Literal))>();

    println!("tokens: {:#?}", ident);
}
