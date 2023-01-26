use lang_lexing::{Extract, Span};

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
    fn extract(
        token: &'a str,
        span: Span,
        cursor: &mut lang_lexing::Cursor<'a>,
    ) -> lang_lexing::Result<'a, T> {
        if (token.as_bytes()[0] as char).is_ascii_punctuation() {
            let spacing = if let Some((_, next)) = cursor.peek() {
                if (next.as_bytes()[0] as char).is_ascii_punctuation() {
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
                span,
            }
            .into())
        } else {
            Err(cursor.error("expected punctionation"))
        }
    }
}
