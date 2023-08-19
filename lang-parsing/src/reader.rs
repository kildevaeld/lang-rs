use super::{error::Error, parse::Parse, peek::Peek};
use crate::ErrorKind;
use alloc::vec::Vec;
use lang_lexing::{TokenRef, WithSpan};

#[derive(Debug)]
pub struct TokenReader<'a, 'b, T> {
    pub(crate) input: &'a str,
    pub(crate) tokens: &'b Vec<T>,
    pub(crate) current: &'b mut usize,
}
// impl<'a, 'b, T> Clone for TokenReader<'a, 'b, T> {
//     fn clone(&self) -> Self {
//         TokenReader {
//             input: self.input,
//             tokens: self.tokens,
//             current: self.current,
//         }
//     }
// }

impl<'a, 'b, T> TokenReader<'a, 'b, T> {
    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn is_empty(&self) -> bool {
        *self.current >= self.tokens.len()
    }

    pub fn current(&self) -> Option<&T> {
        self.tokens.get(*self.current)
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
        if offset >= self.tokens.len() {
            return false;
        }

        P::peek(TokenReader {
            input: self.input,
            tokens: self.tokens,
            current: &mut (*self.current + offset),
        })
    }

    pub fn peek_while<P: Peek<'a, T>, N: Peek<'a, T>>(&mut self) -> bool {
        let mut i = 1;
        while self.peek_offset::<N>(i) {
            i += 1;
        }
        self.peek_offset::<P>(i)
    }

    pub fn eat<P>(&mut self) -> Result<(), Error>
    where
        P: Parse<'a, T>,
    {
        self.parse::<P>().map(|_| ())
    }

    pub fn eat_while<P>(&mut self) -> Result<(), Error>
    where
        P: Parse<'a, T> + Peek<'a, T>,
    {
        while self.peek::<P>() {
            self.eat::<P>()?;
        }

        Ok(())
    }

    pub fn take<I>(&mut self) -> Option<I>
    where
        T: TokenRef<I>,
        I: Clone,
    {
        self.next(|item| item.value().cloned())
    }

    fn next<F, R>(&mut self, func: F) -> Option<R>
    where
        F: FnOnce(&T) -> Option<R>,
    {
        match self.tokens.get(*self.current) {
            Some(found) => {
                if let Some(ret) = func(found) {
                    *self.current += 1;
                    Some(ret)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn step<F, R>(&mut self, func: F) -> Result<R, Error>
    where
        F: FnMut(TokenReader<'a, '_, T>) -> Result<R, Error>,
    {
        self.child::<F, R>(func)
    }

    fn child<F, R>(&mut self, mut func: F) -> Result<R, Error>
    where
        F: FnMut(TokenReader<'a, '_, T>) -> Result<R, Error>,
    {
        let mut child_idx = *self.current;

        match func(TokenReader {
            input: self.input,
            tokens: self.tokens,
            current: &mut child_idx,
        }) {
            Ok(ret) => {
                *self.current = child_idx;
                Ok(ret)
            }
            e => e,
        }
    }

    pub fn offset(&self, offset: isize) -> Option<&T> {
        let len = self.tokens.len() as isize;
        let idx = *self.current as isize;
        let new_idx = idx + offset;
        let new_idx = if new_idx < 0 {
            0
        } else if new_idx > len {
            (len as usize) - 1
        } else {
            new_idx as usize
        };

        Some(&self.tokens[new_idx])
    }

    pub fn error(&self, error: impl Into<ErrorKind>) -> Error
    where
        T: WithSpan,
    {
        let span = self
            .tokens
            .get(*self.current)
            .map(|token| token.span())
            .unwrap_or_default();

        Error::new(error, span)
    }
}
