use crate::{cursor::ChildCursor, Error, Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralBinary;

fn parse<'a>(pos: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, Literal<'a>> {
    while let Some((next_span, next_token)) = cursor.next() {
        match next_token {
            "\\" => {
                let (_escape_pos, escape) = match cursor.peek() {
                    Some(ret) => ret,
                    None => return Err(Error::new(next_span, "unterminated string")),
                };
                match escape.as_bytes()[0] {
                    b'\\' | b'n' | b't' | b'"' => {
                        cursor.next();
                        continue;
                    }
                    _i => return Err(Error::new(next_span, "invalid escape character")),
                }
            }
            "\"" => {
                let span = Span::new(pos, next_span + 1);
                let lexeme = span
                    .slice(cursor.input())
                    .ok_or_else(|| Error::new(next_span, "invalid span"))?;
                return Ok(Literal {
                    lexeme,
                    span,
                    kind: LiteralType::Binary,
                });
            }
            _ => {
                continue;
            }
        }
    }

    Err(Error::new(pos, "unterminated string literal"))
}

impl<'a, T: From<Literal<'a>>> Extract<'a, T> for LiteralBinary {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token == "b" {
            if let Some((_, c)) = cursor.peek() {
                if c == "\"" {
                    let _ = cursor.next();
                    return parse(span, cursor).map(|i| i.into());
                }
            }
        }

        Err(Error::new(span, "string literal"))
    }
}

#[cfg(test)]
mod test {
    use crate::{tokens::Token, TokenRef};

    use super::*;

    #[test]
    fn test_binary_literal() {
        use crate::Lexer;

        let lexer = Lexer::<LiteralBinary, Token<'_>>::new(r#"b"binary string!""#);

        let ret = lexer.tokenize().next().unwrap().expect("ok");

        let Token::Literal(lit) = ret else {
            panic!("literal")
        };

        assert_eq!(lit.kind, LiteralType::Binary);
        assert_eq!(lit.lexeme, r#"b"binary string!""#);
    }
}
