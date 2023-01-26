use crate::{cursor::ChildCursor, string_ext::StringExt, Error, Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralNumber;

pub fn parse_number<'a>(
    span: usize,
    _token: &'a str,
    cursor: &mut ChildCursor<'a, '_>,
) -> Result<'a, Literal<'a>> {
    let mut float = false;

    let mut last_span = span;

    while let Some((next_span, next_token)) = cursor.peek() {
        if next_token.is_digit() {
            cursor.next();
            last_span = next_span;
            continue;
        } else if next_token == "." {
            if float {
                break;
            }
            float = true;
            last_span = next_span;
            cursor.next();
        } else {
            break;
        }
    }

    let span = Span::new(span, last_span + 1);

    let lexeme = span
        .slice(cursor.input())
        .ok_or_else(|| Error::new(span.start, "invalid span"))?;

    Ok(Literal {
        lexeme,
        span,
        kind: if float {
            LiteralType::Float
        } else {
            LiteralType::Integer
        },
    })
}

impl<'a, T: From<Literal<'a>>> Extract<'a, T> for LiteralNumber {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token.is_digit() {
            parse_number(span, token, cursor).map(|i| i.into())
        } else {
            Err(Error::new(span, "number literal"))
        }
    }
}
