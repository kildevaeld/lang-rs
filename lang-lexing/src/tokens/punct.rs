use crate::{cursor::ChildCursor, string_ext::StringExt, Error, Extract, Result, Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Spacing {
    Alone,
    Joint,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Punct<'a> {
    pub lexeme: &'a str,
    pub spacing: Spacing,
    pub span: Span,
}

impl<'a, T: From<Punct<'a>>> Extract<'a, T> for Punct<'a> {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token.is_ascii_punctuation() {
            let spacing = if let Some((_, next)) = cursor.peek() {
                if next.is_ascii_punctuation() && next != "_" {
                    Spacing::Joint
                } else {
                    Spacing::Alone
                }
            } else {
                Spacing::Alone
            };

            Ok(Punct {
                lexeme: token,
                spacing,
                span: Span::new(span, span + token.len()),
            }
            .into())
        } else {
            Err(Error::new(span, "punctuation"))
        }
    }
}

impl<'a> WithSpan for Punct<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
