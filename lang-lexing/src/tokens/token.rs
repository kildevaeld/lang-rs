use super::{whitespace::Whitespace, Comment, Ident, Literal, LiteralNumber, LiteralString, Punct};
use crate::{Lexer, LexerFactory, Span, TokenRef, WithSpan};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token<'a> {
    Ident(#[cfg_attr(feature = "serde", serde(borrow))] Ident<'a>),
    Punct(Punct<'a>),
    Literal(Literal<'a>),
    Whitespace(Whitespace<'a>),
    Comment(Comment<'a>),
}

impl<'a> LexerFactory<'a, Token<'a>> for Token<'a> {
    type Extract = (
        LiteralNumber,
        // LiteralBool,
        LiteralString,
        Ident<'a>,
        Punct<'a>,
        Whitespace<'a>,
    );
    fn create_lexer(input: &'a str) -> Lexer<'a, Self::Extract, Token<'a>> {
        Lexer::new(input)
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

impl<'a> From<Whitespace<'a>> for Token<'a> {
    fn from(value: Whitespace<'a>) -> Self {
        Token::Whitespace(value)
    }
}

impl<'a> From<Comment<'a>> for Token<'a> {
    fn from(value: Comment<'a>) -> Self {
        Token::Comment(value)
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

impl<'a> TokenRef<Whitespace<'a>> for Token<'a> {
    fn value(&self) -> Option<&Whitespace<'a>> {
        match self {
            Token::Whitespace(ident) => Some(ident),
            _ => None,
        }
    }
}

impl<'a> TokenRef<Comment<'a>> for Token<'a> {
    fn value(&self) -> Option<&Comment<'a>> {
        match self {
            Token::Comment(ident) => Some(ident),
            _ => None,
        }
    }
}

impl<'a> WithSpan for Token<'a> {
    fn span(&self) -> Span {
        match self {
            Token::Ident(ident) => ident.span,
            Token::Literal(lit) => lit.span,
            Token::Punct(punct) => punct.span,
            Token::Whitespace(whitespace) => whitespace.span,
            Token::Comment(comment) => comment.span,
        }
    }
}
