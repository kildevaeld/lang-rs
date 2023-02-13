use crate::{cursor::ChildCursor, string_ext::StringExt, Error, Extract, Result, Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Whitespace<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a, T: From<Whitespace<'a>>> Extract<'a, T> for Whitespace<'a> {
    #[inline]
    fn extract(token: &'a str, span: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if token.is_whitespace() {
            let mut last_span = span;
            while let Some((next_pos, next_token)) = cursor.peek() {
                if next_token.is_whitespace() {
                    last_span = next_pos;
                    cursor.next();
                } else {
                    break;
                }
            }

            let span = Span::new(span, last_span + 1);

            Ok(Whitespace {
                lexeme: span.slice(cursor.input()).expect("lexeme"),
                span,
            }
            .into())
        } else {
            Err(Error::new(span, "whitespace"))
        }
    }
}

impl<'a> WithSpan for Whitespace<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
