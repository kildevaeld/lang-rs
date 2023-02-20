use alloc::boxed::Box;

use crate::{ErrorKind, Parser};

use super::{cursor::Cursor, error::Error, reader::TokenReader, Peek};

pub trait Parse<'a, T>: Sized {
    /// Parse self from a token reader
    /// if this fails, the reader can be in an invalid state
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error>;
}

pub trait Rule {
    type Parse<'a, T>: Parse<'a, T>
    where
        Self: 'a;

    const NAME: &'static str;

    fn expected() -> ErrorKind;
}

macro_rules! parse_impl {
    ($first: ident) => {

        impl<'a, T, $first: Peek<'a, T>> Peek<'a, T> for ($first, ) {
            fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
                $first::peek(cursor)
            }
        }

        impl<'a, T, $first: Parse<'a, T>> Parse<'a, T> for ($first,) {
            fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<($first,), Error> {
                Ok((
                    state.parse()?,
                ))
            }
        }
    };
    ($first: ident $($rest:ident)*) => {
        parse_impl!($($rest)*);

        impl<'a, T, $first: Peek<'a, T>, $($rest),*> Peek<'a, T> for ($first, $($rest),*) {
            fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
                $first::peek(cursor)
            }
        }

        impl<'a, T, $first: Parse<'a, T>, $($rest: Parse<'a, T>),*> Parse<'a, T> for ($first, $($rest),*) {

            fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<($first, $($rest),*), Error> {
                Ok((
                    state.parse()?,
                    $(state.parse::<$rest>()?),*
                ))
            }
        }
    };
}

parse_impl!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);

impl<'a, T, TOKEN> Parse<'a, TOKEN> for Option<T>
where
    T: Parse<'a, TOKEN>,
{
    fn parse(state: &mut TokenReader<'a, '_, TOKEN>) -> Result<Self, Error> {
        match state.parse::<T>() {
            Ok(ret) => Ok(Some(ret)),
            Err(_) => Ok(None),
        }
    }
}

impl<'a, T, TOKEN> Peek<'a, TOKEN> for Option<T>
where
    T: Peek<'a, TOKEN>,
{
    fn peek(cursor: &mut Cursor<'a, '_, TOKEN>) -> bool {
        T::peek(cursor)
    }
}

#[cfg(feature = "either")]
impl<'a, T, L, R> Parse<'a, T> for either::Either<L, R>
where
    L: Parse<'a, T>,
    R: Parse<'a, T>,
    T: lang_lexing::WithSpan,
{
    #[allow(non_snake_case)]
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
        use alloc::{format, vec};
        use either::Either;
        any_of!(state,
            L => l { Either::Left(l) },
            R => r { Either::Right(r) }
        )
    }
}

#[cfg(feature = "either")]
impl<'a, T, L, R> Peek<'a, T> for either::Either<L, R>
where
    L: Peek<'a, T>,
    R: Peek<'a, T>,
    T: lang_lexing::WithSpan,
{
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool {
        L::peek(cursor) || R::peek(cursor)
    }
}

impl<'a, T, TOKEN> Parse<'a, TOKEN> for Box<T>
where
    T: Parse<'a, TOKEN>,
{
    fn parse(state: &mut TokenReader<'a, '_, TOKEN>) -> Result<Self, Error> {
        Ok(Box::new(T::parse(state)?))
    }
}

impl<'a, T, TOKEN> Peek<'a, TOKEN> for Box<T>
where
    T: Peek<'a, TOKEN>,
{
    fn peek(cursor: &mut Cursor<'a, '_, TOKEN>) -> bool {
        T::peek(cursor)
    }
}

impl<'a, T, TOKEN> Peek<'a, TOKEN> for alloc::vec::Vec<T>
where
    T: Peek<'a, TOKEN>,
{
    fn peek(cursor: &mut Cursor<'a, '_, TOKEN>) -> bool {
        T::peek(cursor)
    }
}

impl<'a, T, TOKEN> Parse<'a, TOKEN> for alloc::vec::Vec<T>
where
    T: Parse<'a, TOKEN> + Peek<'a, TOKEN>,
{
    fn parse(state: &mut TokenReader<'a, '_, TOKEN>) -> Result<Self, Error> {
        let mut output = alloc::vec::Vec::new();

        loop {
            if !state.peek::<T>() {
                break;
            }

            output.push(state.parse()?);
        }

        Ok(output)
    }
}
