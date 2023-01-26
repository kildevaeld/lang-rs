use lang_lexing::{Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralString;

fn parse<'a>(pos: Span, cursor: &mut lang_lexing::Cursor<'a>) -> Result<'a, Literal<'a>> {
    loop {
        let (next_post, next) = match cursor.next() {
            Some(next) => next,
            None => break,
        };
        match next.as_bytes()[0] {
            b'\\' => {
                let (_escape_pos, escape) = match cursor.peek() {
                    Some(ret) => ret,
                    None => return Err(cursor.error("unterminated string")),
                };
                match escape.as_bytes()[0] {
                    b'\\' | b'n' | b't' | b'"' => {
                        cursor.next();
                        continue;
                    }
                    _i => return Err(cursor.error("invalid escape charecter")),
                }
            }
            b'"' => {
                return Ok(Literal {
                    lexeme: &cursor.input()[pos.start..next_post.end],
                    span: Span::new(pos.start, next_post.end),
                    kind: LiteralType::String,
                })
            }
            _ => {
                continue;
            }
        }
    }

    Err(cursor.error("unterminated string"))
}

impl<'a, T: From<Literal<'a>>> Extract<'a, T> for LiteralString {
    #[inline]
    fn extract(token: &'a str, span: Span, cursor: &mut lang_lexing::Cursor<'a>) -> Result<'a, T> {
        match token.as_bytes()[0] {
            b'"' => parse(span, cursor).map(|i| i.into()),
            _ => Err(cursor.error("expected literal")),
        }
    }
}
