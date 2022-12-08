use super::{error::Error, reader::TokenReader};

pub trait Parse<'a, T>: Sized {
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error>;
}

macro_rules! parse_impl {
    ($first: ident) => {
        impl<'a, T, $first: Parse<'a, T>> Parse<'a, T> for ($first,) {
            fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<($first,), Error> {
                Ok((
                    $first::parse(state)?,
                ))
            }
        }
    };
    ($first: ident $($rest:ident)*) => {
        parse_impl!($($rest)*);

        impl<'a, T, $first: Parse<'a, T>, $($rest: Parse<'a, T>),*> Parse<'a, T> for ($first, $($rest),*) {

            fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<($first, $($rest),*), Error> {
                Ok((
                    $first::parse(state)?,
                    $($rest::parse(state)?),*
                ))
            }
        }
    };
}

parse_impl!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);

impl<'a, T> Parse<'a, T> for Option<T>
where
    T: Parse<'a, T>,
{
    fn parse(state: &mut TokenReader<'a, '_, T>) -> Result<Self, Error> {
        match state.parse::<T>() {
            Ok(ret) => Ok(Some(ret)),
            Err(_) => Ok(None),
        }
    }
}
/*
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'a, L, R> Parse<'a> for Either<L, R>
where
    L: Parse<'a>,
    R: Parse<'a>,
{
    fn parse(state: &mut TokenReader<'a, '_>) -> Result<Self, Error> {
        match L::parse(state) {
            Ok(ret) => Ok(Either::Left(ret)),
            Err(left_err) => match R::parse(state) {
                Ok(ret) => Ok(Either::Right(ret)),
                Err(right_err) => Err(Error::new(
                    alloc::format!("expected {} or {}", left_err, right_err),
                    right_err.span,
                )),
            },
        }
    }
}

impl<'a, T> Parse<'a> for Vec<T>
where
    T: Parse<'a>,
{
    fn parse(state: &mut TokenReader<'a, '_>) -> Result<Self, Error> {
        let mut items = Vec::default();

        loop {
            if state.is_empty() {
                break;
            }

            let item = state.parse::<T>()?;

            items.push(item);
        }

        Ok(items)
    }
}*/
