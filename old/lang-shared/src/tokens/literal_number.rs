use lang_lexing::{Error, Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralNumber;

pub fn parse_number<'a>(span: Span, token: &'a str) -> Result<'a, Literal<'a>> {
    let mut float = false;
    for c in token.chars() {
        if c.is_numeric() {
            continue;
        } else if c == '.' {
            if float {
                return Err(Error {
                    message: "number cannot contain multiple fractions".into(),
                    span,
                });
            }
            float = true
        } else {
            return Err(Error {
                message: "invalid format".into(),
                span,
            });
        }
    }

    Ok(Literal {
        lexeme: token,
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
    fn extract(token: &'a str, span: Span, cursor: &mut lang_lexing::Cursor<'a>) -> Result<'a, T> {
        match token.as_bytes()[0] {
            b'1'..=b'9' => parse_number(span, token).map(|i| i.into()),
            _ => Err(cursor.error("expected number")),
        }
    }
}
