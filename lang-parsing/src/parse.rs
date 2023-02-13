use super::{error::Error, reader::TokenReader};

pub trait Parse<'a, T>: Sized {
    /// Parse self from a token reader
    /// if this fails, the reader can be in an invalid state
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error>;
}

macro_rules! parse_impl {
    ($first: ident) => {
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

#[cfg(feature = "either")]
impl<'a, T, L, R> Parse<'a, T> for either::Either<L, R>
where
    L: Parse<'a, T>,
    R: Parse<'a, T>,
    T: WithSpan,
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
