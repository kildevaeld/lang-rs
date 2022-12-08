```rust
use lang_lexing::{utils, Cursor, Error, Extract, Lexer, Span};

#[derive(Debug)]
pub struct Punct<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a, O: From<Self>> Extract<'a, O> for Punct<'a> {
    fn extract(token: &'a str, span: Span, cursor: &mut Cursor<'a>) -> Result<O, Error<'a>> {
        if utils::is_ascii_punctuation(token) {
            Ok(Punct {
                lexeme: token,
                span,
            }
            .into())
        } else {
            Err(cursor.error("expected punctuation"))
        }
    }
}

fn main() {
    let lexer = Lexer::<Punct, Punct>::new("fn test() { mig }");

    let tokens = lexer
        .tokenize()
        .filter_map(|ret| ret.ok())
        .collect::<Vec<_>>();

    println!("tokens: {:#?}", tokens);
}



```
