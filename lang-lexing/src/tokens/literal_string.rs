use crate::{cursor::ChildCursor, Error, Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralString;

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
                    kind: LiteralType::String,
                });
            }
            _ => {
                continue;
            }
        }
    }

    Err(Error::new(pos, "unterminated string literal"))
}

impl<'a, T: From<Literal<'a>>> Extract<'a, T> for LiteralString {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token == "\"" {
            parse(span, cursor).map(|i| i.into())
        } else {
            Err(Error::new(span, "string literal"))
        }
    }
}
