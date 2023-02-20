use alloc::{borrow::Cow, string::String, vec::Vec};
use core::fmt;
use lang_lexing::Span;

#[derive(Debug)]
pub enum ErrorKind {
    Expected {
        message: Cow<'static, str>,
        rule: Option<Cow<'static, str>>,
    },
    OneOf {
        errors: Vec<ErrorKind>,
        rule: Option<Cow<'static, str>>,
    },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Expected { message, .. } => {
                write!(f, "{message}")
            }
            ErrorKind::OneOf { errors, .. } => {
                for (idx, next) in errors.iter().enumerate() {
                    if idx > 0 {
                        write!(f, " or ")?;
                    }
                    write!(f, "{next}")?
                }
                Ok(())
            }
        }
    }
}

impl From<(&'static str, &'static str)> for ErrorKind {
    fn from(value: (&'static str, &'static str)) -> Self {
        ErrorKind::Expected {
            message: value.1.into(),
            rule: Some(value.0.into()),
        }
    }
}

impl From<&'static str> for ErrorKind {
    fn from(value: &'static str) -> Self {
        ErrorKind::Expected {
            message: value.into(),
            rule: None,
        }
    }
}

impl From<(String, String)> for ErrorKind {
    fn from(value: (String, String)) -> Self {
        ErrorKind::Expected {
            message: value.1.into(),
            rule: None,
        }
    }
}

impl From<String> for ErrorKind {
    fn from(value: String) -> Self {
        ErrorKind::Expected {
            message: value.into(),
            rule: None,
        }
    }
}

impl<S> From<(S, Vec<ErrorKind>)> for ErrorKind
where
    S: Into<Cow<'static, str>>,
{
    fn from(errors: (S, Vec<ErrorKind>)) -> Self {
        ErrorKind::OneOf {
            errors: errors.1,
            rule: Some(errors.0.into()),
        }
    }
}

// impl<'a, S> From<&'a [S]> for ErrorKind
// where
//     S: Into<ErrorKind> + Copy,
// {
//     fn from(errors: &'a [S]) -> Self {
//         ErrorKind::OneOf {
//             errors: errors.iter().map(|s| (*s).into()).collect(),
//         }
//     }
// }

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

impl Error {
    pub fn new(message: impl Into<ErrorKind>, span: Span) -> Error {
        Error {
            kind: message.into(),
            span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]: {}", self.span.start, self.span.end, self.kind)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
