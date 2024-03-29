use crate::{Span, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LiteralType<'a> {
    String,
    Integer,
    Float,
    Bool,
    Binary,
    Custom(alloc::borrow::Cow<'a, str>),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub lexeme: &'a str,
    pub span: Span,
    pub kind: LiteralType<'a>,
}

impl<'a> WithSpan for Literal<'a> {
    fn span(&self) -> Span {
        self.span
    }
}
