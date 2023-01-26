#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
use core::fmt;

use crate::Span;

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub message: Cow<'a, str>,
    pub span: Span,
}

#[cfg(not(feature = "alloc"))]
#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub message: &'a str,
    pub span: Span,
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.span, self.message)
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}
