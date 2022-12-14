use super::{
    cursor::Cursor,
    error::Error,
    parse::Parse,
    token::{Peek, Token},
};
use alloc::borrow::Cow;
use alloc::vec::Vec;
use lang_lexing::{Span, WithSpan};

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

    pub fn parse<P>(&mut self) -> Result<P, Error>
    where
        P: Parse<'a, T>,
    {
        P::parse(self)
    }

    pub fn peek<P: Peek<'a, T>>(&self, token: P) -> bool {
        self.peek_offset(token, 0)
    }

    pub fn peek2<P: Peek<'a, T>>(&self, token: P) -> bool {
        self.peek_offset(token, 1)
    }

    pub fn peek_offset<P: Peek<'a, T>>(&self, token: P, offset: usize) -> bool {
        let _ = token;
        let mut cursor = Cursor {
            input: self.input,
            tokens: &self.tokens,
            current: self.current + offset,
        };
        P::Token::peek(&mut cursor)
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
            tokens: &self.tokens,
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

    pub fn error(&self, error: impl Into<Cow<'static, str>>) -> Error
    where
        T: WithSpan,
    {
        let span = self
            .tokens
            .get(self.current)
            .map(|token| *token.span())
            .unwrap_or(Span::new(0, 0));
        Error::new(error, span)
    }
}
