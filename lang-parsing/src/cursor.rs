use super::error::Error;
use crate::{ErrorKind, Peek};
use alloc::vec::Vec;
use lang_lexing::{Span, TokenRef, WithSpan};

#[derive(Debug)]
pub struct Cursor<'a, 'b, T> {
    pub(crate) input: &'a str,
    pub(crate) tokens: &'b Vec<T>,
    pub(crate) current: usize,
}

impl<'a, 'b, T> Clone for Cursor<'a, 'b, T> {
    fn clone(&self) -> Self {
        Cursor {
            input: self.input,
            tokens: self.tokens,
            current: self.current,
        }
    }
}

impl<'a, 'b, T> Cursor<'a, 'b, T> {
    /// Current token T
    pub fn current(&self) -> Option<&T> {
        self.tokens.get(self.current)
    }

    pub fn slice(&self) -> Option<&'a str>
    where
        T: WithSpan,
    {
        self.current()
            .and_then(|current| current.span().slice(self.input))
    }

    /// Returns a reference to the input
    pub fn input(&self) -> &'a str {
        self.input
    }

    /// The span of the current token
    pub fn span(&self) -> Span
    where
        T: WithSpan,
    {
        self.current().map(|m| m.span()).unwrap_or_default()
    }

    /// Consume and return the next token
    pub fn take<I>(&mut self) -> Option<I>
    where
        T: TokenRef<I>,
        I: Clone,
    {
        self.next(|item| item.value().cloned())
    }

    pub fn error(&self, error: impl Into<ErrorKind>) -> Error
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

    /// Peek first non N
    /// eg: cursor.peek_while::<Ident, Whitespace>();
    pub fn peek_while<P: Peek<'a, T>, N: Peek<'a, T>>(&mut self) -> bool {
        let mut i = 1;
        while self.peek_offset::<N>(i) {
            i += 1;
        }

        P::peek(&mut self.offset(i as isize).expect("peek"))
    }

    pub fn peek<P: Peek<'a, T>>(&mut self) -> bool {
        P::peek(self)
    }

    pub fn peek2<P: Peek<'a, T>>(&mut self) -> bool {
        self.peek_offset::<P>(1)
    }

    pub fn peek_offset<P: Peek<'a, T>>(&mut self, offset: usize) -> bool {
        P::peek(&mut Cursor {
            input: self.input,
            tokens: self.tokens,
            current: self.current + offset,
        })
    }

    pub fn offset(&self, offset: isize) -> Result<Cursor<'a, 'b, T>, Error> {
        let len = self.tokens.len() as isize;
        let idx = self.current as isize;
        let new_idx = idx + offset;
        let new_idx = if new_idx < 0 {
            0
        } else if new_idx > len {
            (len as usize) - 1
        } else {
            new_idx as usize
        };

        Ok(Cursor {
            input: self.input,
            tokens: self.tokens,
            current: new_idx,
        })
    }
}
