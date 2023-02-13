use alloc::{borrow::Cow, vec::Vec};
use core::fmt;

use crate::Span;

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind<'a> {
    InvalidRange { span: Span },
    Expected { message: Cow<'a, str> },
    AnyOf { errors: Vec<ErrorKind<'a>> },
}

impl<'a> fmt::Display for ErrorKind<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Expected { message } => {
                write!(f, "{message}")
            }
            ErrorKind::InvalidRange { span } => {
                write!(f, "invalid span: {span}")
            }
            ErrorKind::AnyOf { errors } => {
                write!(f, "any of: {errors:?}")
            }
        }
    }
}

impl<'a> From<&'a str> for ErrorKind<'a> {
    fn from(value: &'a str) -> Self {
        ErrorKind::Expected {
            message: value.into(),
        }
    }
}

impl<'a> From<alloc::string::String> for ErrorKind<'a> {
    fn from(value: alloc::string::String) -> Self {
        ErrorKind::Expected {
            message: value.into(),
        }
    }
}

impl<'a> From<Vec<ErrorKind<'a>>> for ErrorKind<'a> {
    fn from(value: Vec<ErrorKind<'a>>) -> Self {
        ErrorKind::AnyOf { errors: value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    pub kind: ErrorKind<'a>,
    pub position: usize,
}

impl<'a> Error<'a> {
    pub fn new<S: Into<ErrorKind<'a>>>(position: usize, message: S) -> Error<'a> {
        Error {
            kind: message.into(),
            position,
        }
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.position, self.kind)
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}
