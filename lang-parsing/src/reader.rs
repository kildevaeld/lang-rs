use super::{cursor::Cursor, error::Error, parse::Parse, peek::Peek};
use crate::ErrorKind;
use alloc::vec::Vec;
use lang_lexing::WithSpan;

#[derive(Debug, Clone)]
pub struct TokenReader<'a, 'b, T> {
    pub(crate) input: &'a str,
    pub(crate) tokens: &'b Vec<T>,
    pub(crate) current: usize,
}

impl<'a, 'b, T> TokenReader<'a, 'b, T> {
    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn is_empty(&self) -> bool {
        self.current >= self.tokens.len()
    }

    pub fn current(&self) -> Option<&T> {
        self.tokens.get(self.current)
    }

    /// Try to parse.
    /// Will reset index pointer on error
    pub fn parse<P>(&mut self) -> Result<P, Error>
    where
        P: Parse<'a, T>,
    {
        self.child(P::parse)
    }

    pub fn peek<P: Peek<'a, T>>(&self) -> bool {
        self.peek_offset::<P>(0)
    }

    pub fn peek2<P: Peek<'a, T>>(&self) -> bool {
        self.peek_offset::<P>(1)
    }

    pub fn peek_offset<P: Peek<'a, T>>(&self, offset: usize) -> bool {
        let mut cursor = Cursor {
            input: self.input,
            tokens: self.tokens,
            current: self.current + offset,
        };
        P::peek(&mut cursor)
    }

    pub fn eat<P>(&mut self) -> Result<(), Error>
    where
        P: Parse<'a, T>,
    {
        self.parse::<P>().map(|_| ())
    }

    pub fn step<F, R>(&mut self, func: F) -> Result<R, Error>
    where
        F: FnOnce(&mut Cursor<'a, 'b, T>) -> Result<R, Error>,
    {
        let mut cursor = Cursor {
            input: self.input,
            tokens: self.tokens,
            current: self.current,
        };

        match func(&mut cursor) {
            Ok(ret) => {
                self.current = cursor.current;
                Ok(ret)
            }
            err => err,
        }
    }

    fn child<F, R>(&mut self, mut func: F) -> Result<R, Error>
    where
        F: FnMut(&mut TokenReader<'a, '_, T>) -> Result<R, Error>,
    {
        let mut child = TokenReader {
            input: self.input,
            tokens: self.tokens,
            current: self.current,
        };

        match func(&mut child) {
            Ok(ret) => {
                self.current = child.current;
                Ok(ret)
            }
            e => e,
        }
    }

    pub fn error(&self, error: impl Into<ErrorKind>) -> Error
    where
        T: WithSpan,
    {
        let span = self
            .tokens
            .get(self.current)
            .map(|token| token.span())
            .unwrap_or_default();

        Error::new(error, span)
    }
}
