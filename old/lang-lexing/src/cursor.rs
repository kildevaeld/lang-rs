use core::{fmt, iter::Peekable};

use crate::Span;

use super::error::Error;
#[cfg(feature = "alloc")]
use alloc::borrow::Cow;

use unicode_segmentation::UWordBoundIndices;

pub struct Cursor<'a> {
    pub(crate) input: &'a str,
    pub(crate) iter: Peekable<UWordBoundIndices<'a>>,
    pub(crate) current_span: Span,
}

impl<'a> fmt::Debug for Cursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cursor")
            .field("input", &self.input)
            .finish()
    }
}

impl<'a> Cursor<'a> {
    #[cfg(feature = "alloc")]
    pub fn error<T>(&self, message: T) -> Error<'a>
    where
        T: Into<Cow<'a, str>>,
    {
        Error {
            span: self.current_span,
            message: message.into(),
        }
    }

    #[cfg(not(feature = "alloc"))]
    pub fn error(&self, message: &'a str) -> Error<'a> {
        Error { message }
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn peek(&mut self) -> Option<(Span, &'a str)> {
        self.iter.peek().map(|m| (Span::from(*m), m.1))
    }

    pub fn next_non_whitespace(&mut self) -> Option<(Span, &'a str)> {
        loop {
            let (pos, next) = match self.next() {
                Some(ret) => ret,
                None => return None,
            };

            if (next.as_bytes()[0] as char).is_whitespace() {
                continue;
            }

            return Some((pos, next));
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = (Span, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.iter.next().map(|m| (Span::from(m), m.1)) {
            Some(ret) => ret,
            None => return None,
        };

        self.current_span = next.0;

        Some(next)
    }
}
