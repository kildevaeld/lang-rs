mod group;
mod punctuated;

pub use self::{group::*, punctuated::*};
use crate::{Cursor, Error, Parse, Peek, TokenReader};
use alloc::string::ToString;
use lang_lexing::{
    tokens::{Comment, Ident, Literal, Punct, Spacing, Whitespace},
    Span, TokenRef, WithSpan,
};
use unicode_segmentation::UnicodeSegmentation;

macro_rules! lex {
    ($($name: ident),*) => {
        $(
            impl<'a, T> Parse<'a, T> for $name<'a> where T: WithSpan + TokenRef<$name<'a>> {
                fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
                    state.step(|cursor| match cursor.take::<$name>() {
                        Some(ret) => Ok(ret),
                        None => Err(cursor.error(stringify!($name))),
                    })
                }
            }
        )*
    };
}

impl<'a, T> Peek<'a, T> for Ident<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

impl<'a, T> Peek<'a, T> for Literal<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

impl<'a, T> Peek<'a, T> for Whitespace<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

impl<'a, T> Peek<'a, T> for Comment<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

lex!(Ident, Punct, Literal, Whitespace, Comment);

pub fn keyword<'a, T>(
    reader: &mut TokenReader<'a, '_, T>,
    keyword: &str,
) -> Result<Ident<'a>, Error>
where
    T: TokenRef<Ident<'a>> + WithSpan,
{
    reader.step(|cursor| {
        if let Some(ident) = cursor.take::<Ident>() {
            if ident.lexeme == keyword {
                return Ok(ident);
            }
        }

        Err(cursor.error(("keyword".to_string(), keyword.to_string())))
    })
}

pub fn keyword_peek<'a, T>(cursor: &mut Cursor<'a, '_, T>, keyword: &str) -> bool
where
    T: TokenRef<Ident<'a>>,
{
    if let Some(token) = cursor.current() {
        if let Some(ident) = token.value() {
            ident.lexeme == keyword
        } else {
            false
        }
    } else {
        false
    }
}

fn punct_helper<'a, T>(cursor: &mut Cursor<'a, '_, T>, token: &str) -> Result<Span, Error>
where
    T: TokenRef<Punct<'a>> + WithSpan,
{
    let mut span: Option<Span> = None;

    for part in token.split_word_bounds() {
        let punct = match cursor.take::<Punct<'a>>() {
            Some(punct) => punct,
            None => return Err(cursor.error(("punctuation".to_string(), token.to_string()))),
        };
        if punct.lexeme != part {
            return Err(cursor.error(("punctuation".to_string(), token.to_string())));
        }

        if let Some(span) = span.as_mut() {
            span.end = punct.span.end;
        } else {
            span = Some(punct.span);
        }
    }

    match span {
        Some(span) => Ok(span),
        None => panic!(""),
    }
}

pub fn punctuation<'a, T>(reader: &mut TokenReader<'a, '_, T>, token: &str) -> Result<Span, Error>
where
    T: TokenRef<Punct<'a>> + WithSpan,
{
    reader.step(|cursor| punct_helper(cursor, token))
}

pub fn punctuation_peek<'a, T>(cursor: &mut Cursor<'a, '_, T>, token: &str) -> bool
where
    T: TokenRef<Punct<'a>> + WithSpan,
{
    punct_helper(&mut cursor.clone(), token).is_ok()
}
