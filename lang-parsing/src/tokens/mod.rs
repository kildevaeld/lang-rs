mod group;
mod punctuated;
mod ws;

pub use self::{group::*, punctuated::*, ws::*};
use crate::{Error, Parse, Peek, TokenReader};
use alloc::string::ToString;
use lang_lexing::{
    tokens::{Comment, Ident, Literal, Punct, Whitespace},
    Span, TokenRef, WithSpan,
};
use unicode_segmentation::UnicodeSegmentation;

macro_rules! lex {
    ($($name: ident),*) => {
        $(
            impl<'a, T> Parse<'a, T> for $name<'a> where T: WithSpan + TokenRef<$name<'a>> + TokenRef<Whitespace<'a>> {
                fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
                    state.eat_while::<Whitespace>()?;
                    state.step(|cursor| match cursor.take::<$name>() {
                        Some(ret) => Ok(ret),
                        None => Err(cursor.error(stringify!($name))),
                    })
                }
            }

            impl<'a, T> Peek<'a, T> for $name<'a>
            where
                T: TokenRef<Self> + TokenRef<Comment<'a>> + TokenRef<Whitespace<'a>>,
            {
                fn peek(cursor: &mut TokenReader<'a, '_, T>) -> bool {
                    let mut offset = 0;
                    while cursor.peek_offset::<Whitespace>(offset) || cursor.peek_offset::<Comment>(offset) {
                        offset += 1;
                    }

                    let Ok(offset_cursor) = cursor.offset(offset as isize) else {
                        return false
                    };



                    match offset_cursor.current() {
                        None => false,
                        Some(current) => {
                            <T as TokenRef<Self>>::value(current).is_some()
                        },
                    }
                }
            }
        )*
    };
}

// impl<'a, T> Peek<'a, T> for Ident<'a>
// where
//     T: TokenRef<Self> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>>,
// {
//     fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
//         let mut offset = 0;
//         while cursor.peek_offset::<Whitespace>(offset) || cursor.peek_offset::<Comment>(offset) {
//             offset += 1;
//         }

//         match cursor.current() {
//             None => false,
//             Some(current) => <T as TokenRef<Self>>::value(current).is_some(),
//         }
//     }
// }

// impl<'a, T> Parse<'a, T> for Ident<'a>
// where
//     T: TokenRef<Self> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
// {
//     fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
//         state.eat_while::<Whitespace>()?;
//         state.step(|cursor| match cursor.take::<Self>() {
//             Some(ret) => Ok(ret),
//             None => Err(cursor.error("identifier")),
//         })
//     }
// }

// impl<'a, T> Peek<'a, T> for Literal<'a>
// where
//     T: TokenRef<Self>,
// {
//     fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
//         match cursor.current() {
//             None => false,
//             Some(current) => current.value().is_some(),
//         }
//     }
// }

impl<'a, T> Peek<'a, T> for Whitespace<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut TokenReader<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

impl<'a, T> Parse<'a, T> for Whitespace<'a>
where
    T: TokenRef<Self> + WithSpan,
{
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
        state.step(|cursor| match cursor.take::<Self>() {
            Some(ret) => Ok(ret),
            None => Err(cursor.error("whitespace")),
        })
    }
}

impl<'a, T> Peek<'a, T> for Comment<'a>
where
    T: TokenRef<Self>,
{
    fn peek(cursor: &mut TokenReader<'a, '_, T>) -> bool {
        match cursor.current() {
            None => false,
            Some(current) => current.value().is_some(),
        }
    }
}

impl<'a, T> Parse<'a, T> for Comment<'a>
where
    T: TokenRef<Self> + WithSpan,
{
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
        state.step(|cursor| match cursor.take::<Self>() {
            Some(ret) => Ok(ret),
            None => Err(cursor.error("comment")),
        })
    }
}

lex!(Ident, Punct, Literal);

pub fn keyword<'a, T>(
    reader: &mut TokenReader<'a, '_, T>,
    keyword: &str,
) -> Result<Ident<'a>, Error>
where
    T: TokenRef<Ident<'a>> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    let ident = reader
        .parse::<Ident>()
        .map_err(|_| reader.error(("keyword".to_string(), keyword.to_string())))?;

    if ident.lexeme == keyword {
        Ok(ident)
    } else {
        Err(reader.error(("keyword".to_string(), keyword.to_string())))
    }

    // reader.step(|cursor| {
    //     if let Some(ident) = cursor.take::<Ident>() {
    //         if ident.lexeme == keyword {
    //             return Ok(ident);
    //         }
    //     }

    //     Err(cursor.error(("keyword".to_string(), keyword.to_string())))
    // })
}

pub fn keyword_peek<'a, T>(cursor: &mut TokenReader<'a, '_, T>, keyword: &str) -> bool
where
    T: TokenRef<Ident<'a>> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    let ident = match Ident::parse(&mut cursor.clone()) {
        Ok(i) => i,
        Err(_) => return false,
    };
    ident.lexeme == keyword
}

fn punct_helper<'a, T>(cursor: &mut TokenReader<'a, '_, T>, token: &str) -> Result<Span, Error>
where
    T: TokenRef<Punct<'a>> + TokenRef<Whitespace<'a>> + WithSpan,
{
    let mut span: Option<Span> = None;

    cursor.eat_while::<Whitespace>()?;

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
    T: TokenRef<Punct<'a>> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    reader.step(|cursor| punct_helper(cursor, token))
}

pub fn punctuation_peek<'a, T>(cursor: &mut TokenReader<'a, '_, T>, token: &str) -> bool
where
    T: TokenRef<Punct<'a>> + TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    punct_helper(&mut cursor.clone(), token).is_ok()
}
