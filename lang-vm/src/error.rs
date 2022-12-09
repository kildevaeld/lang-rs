use core::fmt;

use crate::thread::ThreadMode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidStackIndex { index: isize },
    BadThread(BadThreadMode),
}

impl fmt::Display for Error {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct BadThreadMode {
    pub expected: Option<ThreadMode>,
    pub found: ThreadMode,
}

impl fmt::Display for BadThreadMode {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl From<BadThreadMode> for Error {
    fn from(err: BadThreadMode) -> Self {
        Error::BadThread(err)
    }
}
