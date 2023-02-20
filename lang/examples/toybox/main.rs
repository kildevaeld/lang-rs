use lang_lexing::Lexer;
use lang_parsing::Parser;
use stmt::Stmt;

#[macro_use]
mod tokens;

mod exprs;
mod stmt;

fn main() {
    let input = include_str!("fib.test");

    let lexer = tokens::Lexer::new(input);

    let mut parser =
        Parser::from_tokens(input, lexer.skip_whitespace(true).tokenize()).expect("lex");

    let stmts = parser.parse::<Stmt>().expect("message");

    println!("{:#?}", stmts);
}
