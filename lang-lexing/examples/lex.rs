use lang_lexing::{tokens::Token, LexerFactory};

const INPUT: &str = r#"
fn test() {
    "Hello" + "World!" + "\n\" + 200 + 2003.1232
}
"#;

const STRING: &str = r#"
"Hello, World!
"#;

const NUMBER: &str = r#"
1 200 4003.0
"#;

const IDENT: &str = r#"
test _test TeSds  😀sds
"#;

fn main() {
    let lexer = Token::create_lexer(IDENT);

    let tokens = lexer
        .tokenize()
        // .filter_map(|ret| ret.ok())
        .collect::<Vec<_>>();

    println!("tokens: {:#?}", tokens);
}
