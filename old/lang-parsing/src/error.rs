use alloc::borrow::Cow;
use core::fmt;

use lang_lexing::Span;

#[derive(Debug)]
pub struct Error {
    pub message: Cow<'static, str>,
    pub span: Span,
}

impl Error {
    pub fn new(message: impl Into<Cow<'static, str>>, span: Span) -> Error {
        Error {
            message: message.into(),
            span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}]: {}",
            self.span.start, self.span.end, self.message
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
