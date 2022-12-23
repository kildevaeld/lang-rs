use lang_lexing::Span;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LiteralType<'a> {
    String,
    Integer,
    Float,
    Bool,
    Custom(&'a str),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal<'a> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub lexeme: &'a str,
    pub span: Span,
    pub kind: LiteralType<'a>,
}
