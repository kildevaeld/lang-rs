use lang_lexing::{Extract, Result, Span};

use super::{Literal, LiteralType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LiteralBool;

impl<'a, T: From<Literal<'a>>> Extract<'a, T> for LiteralBool {
    #[inline]
    fn extract(token: &'a str, span: Span, cursor: &mut lang_lexing::Cursor<'a>) -> Result<'a, T> {
        match token {
            "true" | "false" => Ok(Literal {
                lexeme: token,
                span,
                kind: LiteralType::Bool,
            }
            .into()),
            _ => Err(cursor.error("expected true or false")),
        }
    }
}
