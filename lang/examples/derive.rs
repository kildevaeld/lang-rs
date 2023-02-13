use lang::{parsing::Parser, Parse};
use lang_lexing::{
    tokens::{Ident, Literal, LiteralNumber, LiteralString, Punct, Token},
    LexerFactory,
};

lang::tokens!(
    Token
    puncts {
        "+" Add

    }
    keywords {
        "fn" Func
    }
    literal { LiteralString, LiteralNumber }
);

#[derive(Debug, Parse)]
pub struct Test<'a> {
    ident: Ident<'a>,
    operator_token: lang_lexing::tokens::Punct<'a>,
    literal: Literal<'a>,
}

#[derive(Debug, Parse)]
pub enum TestEnum<'a> {
    #[parse("function")]
    Other {
        fn_token: Token![fn],
        ident: Ident<'a>,
    },
    Test(Test<'a>),
}

fn main() {
    let input = "test = 2020";

    let lexer = Token::create_lexer(input).skip_whitespace(true);

    let mut parser = Parser::<Token>::from_tokens(input, lexer.tokenize()).expect("parser");

    let test = parser.parse::<TestEnum>().expect("test");

    println!("{test:#?}");
}
