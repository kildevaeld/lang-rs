use lang_lexing::{Extract, Span};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ident<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a, T: From<Self>> Extract<'a, T> for Ident<'a> {
    #[inline]
    fn extract(
        token: &'a str,
        span: Span,
        cursor: &mut lang_lexing::Cursor<'a>,
    ) -> lang_lexing::Result<'a, T> {
        match token.as_bytes()[0] {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(Ident {
                lexeme: token,
                span,
            }
            .into()),
            _ => Err(cursor.error("expected ident")),
        }
    }
}
