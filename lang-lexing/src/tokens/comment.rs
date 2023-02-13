use crate::{Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Comment<'a> {
    pub lexeme: &'a str,
    pub span: Span,
}

impl<'a> WithSpan for Comment<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
