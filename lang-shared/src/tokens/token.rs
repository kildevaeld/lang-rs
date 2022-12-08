use lang_lexing::{Lexer, LexerFactory, Span, TokenRef, WithSpan};

use super::{literal_bool::LiteralBool, Ident, Literal, LiteralNumber, LiteralString, Punct};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Ident(Ident<'a>),
    Punct(Punct<'a>),
    Literal(Literal<'a>),
}

impl<'a> LexerFactory<'a> for Token<'a> {
    type Extract = (
        LiteralNumber,
        LiteralBool,
        LiteralString,
        Ident<'a>,
        Punct<'a>,
    );
    fn create_lexer(input: &'a str) -> Lexer<'a, Self::Extract, Self> {
        Lexer::new(input).skip_whitespace(true)
    }
}

impl<'a> From<Ident<'a>> for Token<'a> {
    fn from(ident: Ident<'a>) -> Self {
        Token::Ident(ident)
    }
}

impl<'a> From<Punct<'a>> for Token<'a> {
    fn from(ident: Punct<'a>) -> Self {
        Token::Punct(ident)
    }
}

impl<'a> From<Literal<'a>> for Token<'a> {
    fn from(ident: Literal<'a>) -> Self {
        Token::Literal(ident)
    }
}

impl<'a> TokenRef<Ident<'a>> for Token<'a> {
    fn value(&self) -> Option<&Ident<'a>> {
        match self {
            Token::Ident(ident) => Some(ident),
            _ => None,
        }
    }
}

impl<'a> TokenRef<Punct<'a>> for Token<'a> {
    fn value(&self) -> Option<&Punct<'a>> {
        match self {
            Token::Punct(ident) => Some(ident),
            _ => None,
        }
    }
}

impl<'a> TokenRef<Literal<'a>> for Token<'a> {
    fn value(&self) -> Option<&Literal<'a>> {
        match self {
            Token::Literal(ident) => Some(ident),
            _ => None,
        }
    }
}

impl<'a> WithSpan for Token<'a> {
    fn span(&self) -> &Span {
        match self {
            Token::Ident(ident) => &ident.span,
            Token::Literal(lit) => &lit.span,
            Token::Punct(punct) => &punct.span,
        }
    }
}
