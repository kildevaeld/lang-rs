use lang_lexing::Span;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LiteralType {
    String,
    Integer,
    Float,
    Bool,
    Custom(&'static str),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal<'a> {
    pub lexeme: &'a str,
    pub span: Span,
    pub kind: LiteralType,
}
