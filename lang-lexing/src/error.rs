use alloc::borrow::Cow;
use core::fmt;

pub type Result<'a, T> = core::result::Result<T, Error<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Error<'a> {
    pub message: Cow<'a, str>,
    pub position: usize,
}

impl<'a> Error<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(position: usize, message: S) -> Error<'a> {
        Error {
            message: message.into(),
            position,
        }
    }
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.position, self.message)
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}
