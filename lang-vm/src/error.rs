use crate::thread::ThreadMode;
use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidStackIndex { index: isize },
    BadThread(BadThreadMode),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadThread(inner) => {
                write!(f, "bad thread error: {inner}")
            }
            Error::InvalidStackIndex { index } => {
                write!(f, "invalid stack: {index}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::BadThread(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BadThreadMode {
    pub expected: Option<ThreadMode>,
    pub found: ThreadMode,
}

impl fmt::Display for BadThreadMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expected {:?}, found: {:?}", self.expected, self.found)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BadThreadMode {}

impl From<BadThreadMode> for Error {
    fn from(err: BadThreadMode) -> Self {
        Error::BadThread(err)
    }
}
