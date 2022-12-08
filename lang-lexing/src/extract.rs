use crate::{cursor::Cursor, error::Result, Span};

pub trait Extract<'a, O>: Sized {
    fn extract(token: &'a str, span: Span, cursor: &mut Cursor<'a>) -> Result<'a, O>;
}

#[cfg(feature = "alloc")]
macro_rules! extract {
    ($first: ident) => {
        impl<'a, $first: Extract<'a, O>, O> Extract<'a, O> for ($first,) {
            fn extract(
                token: &'a str,
                position: Span,
                cursor: &mut Cursor<'a>,
            ) -> Result<'a, O> {
                $first::extract(token, position, cursor)
            }
        }
    };
    ($first: ident $($rest: ident)*) => {
        extract!($($rest)*);

        #[allow(non_snake_case)]
        impl<'a, O, $first, $($rest),*>  Extract<'a, O> for ($first, $($rest),*)
        where
            $first: Extract<'a, O>,
            $(
                $rest: Extract<'a, O>
            ),*
        {

            #[inline]
            fn extract(
                token: &'a str,
                position: Span,
                cursor: &mut Cursor<'a>,
            ) -> Result<'a, O> {
                let $first = match $first::extract(token, position, cursor) {
                    Ok(ret) => return Ok(ret),
                    Err(err) => err,
                };

                $(
                    let $rest = match $rest::extract(token, position, cursor) {
                        Ok(ret) => return Ok(ret),
                        Err(err) => err
                    };
                )*

                let errors = alloc::vec![
                    $first.message,
                    $($rest.message),*
                ];

                Err(cursor.error(alloc::format!("expected one of: {}", errors.join(","))))
            }
        }
    }
}

#[cfg(feature = "alloc")]
extract!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22);
