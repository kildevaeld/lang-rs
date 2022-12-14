use super::error::Error;
use lang_lexing::{Span, TokenRef, WithSpan};

use alloc::{borrow::Cow, vec::Vec};

#[derive(Debug)]
pub struct Cursor<'a, 'b, T> {
    pub(crate) input: &'a str,
    pub(crate) tokens: &'b Vec<T>,
    pub(crate) current: usize,
}

impl<'a, 'b, T> Clone for Cursor<'a, 'b, T> {
    fn clone(&self) -> Self {
        Cursor {
            input: self.input.clone(),
            tokens: self.tokens.clone(),
            current: self.current,
        }
    }
}

impl<'a, 'b, T> Cursor<'a, 'b, T> {
    pub fn current(&self) -> Option<&T> {
        self.tokens.get(self.current)
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn span(&self) -> Span
    where
        T: WithSpan,
    {
        self.current().map(|m| *m.span()).unwrap_or(Span::new(0, 0))
    }

    pub fn take<I>(&mut self) -> Option<I>
    where
        T: TokenRef<I>,
        I: Clone,
    {
        self.next(|item| item.value().map(|item| item.clone()))
    }

    pub fn error(&self, error: impl Into<Cow<'static, str>>) -> Error
    where
        T: WithSpan,
    {
        Error::new(error, self.span())
    }

    fn next<F, R>(&mut self, func: F) -> Option<R>
    where
        F: FnOnce(&T) -> Option<R>,
    {
        match self.tokens.get(self.current) {
            Some(found) => {
                if let Some(ret) = func(found) {
                    self.current += 1;
                    Some(ret)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
