use crate::{cursor::ChildCursor, Error, Extract, Result, Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a, T: From<Self>> Extract<'a, T> for Ident<'a> {
    #[inline]
    fn extract(token: &'a str, pos: usize, cursor: &mut ChildCursor<'a, '_>) -> Result<'a, T> {
        if !token.chars().all(|m| m.is_alphabetic() || m == '_') {
            return Err(Error::new(pos, "identifier"));
        }

        let mut last_span = pos;

        while let Some((next_span, next_token)) = cursor.peek() {
            if !next_token.chars().all(|m| m.is_alphanumeric() || m == '_') {
                break;
            }
            let _ = cursor.next();
            last_span = next_span + next_token.len();
        }

        let span = Span::new(pos, last_span);

        let lexeme = span
            .slice(cursor.input())
            .ok_or_else(|| Error::new(span.start, "invalid range"))?;

        Ok(Ident { lexeme, span }.into())
    }
}

impl<'a> WithSpan for Ident<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
