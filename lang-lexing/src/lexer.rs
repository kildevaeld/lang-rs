use crate::{cursor::Cursor, Extract, Result};
use core::{fmt, marker::PhantomData};

pub struct Lexer<'a, T, O> {
    input: &'a str,
    iter: Cursor<'a>,
    skip_whitespace: bool,
    _o: PhantomData<O>,
    _t: PhantomData<T>,
}

impl<'a, T, O> fmt::Debug for Lexer<'a, T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("skip_whitespace", &self.skip_whitespace)
            .finish()
    }
}

impl<'a, T, O> Lexer<'a, T, O> {
    pub fn new(input: &'a str) -> Lexer<'a, T, O> {
        Lexer {
            input,
            iter: Cursor::new(input, true),
            skip_whitespace: false,
            _o: PhantomData,
            _t: PhantomData,
        }
    }

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn skip_whitespace(mut self, skip: bool) -> Self {
        self.skip_whitespace = skip;
        self
    }
}

impl<'a, T, O> Lexer<'a, T, O>
where
    T: Extract<'a, O>,
{
    pub fn tokenize(self) -> LexerIterator<'a, T, O> {
        LexerIterator {
            iter: self.iter,
            skip_whitespace: self.skip_whitespace,
            _o: PhantomData,
            _t: PhantomData,
        }
    }
}

impl<'a, T, O> IntoIterator for Lexer<'a, T, O>
where
    T: Extract<'a, O>,
{
    type Item = Result<'a, O>;

    type IntoIter = LexerIterator<'a, T, O>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokenize()
    }
}

pub struct LexerIterator<'a, T, O> {
    iter: Cursor<'a>,
    skip_whitespace: bool,
    _o: PhantomData<O>,
    _t: PhantomData<T>,
}

impl<'a, T, O> LexerIterator<'a, T, O> {
    pub fn next_non_whitespace(&mut self) -> Option<(usize, &'a str)> {
        loop {
            let (pos, next) = match self.iter.next_item() {
                Some(ret) => ret,
                None => return None,
            };

            if (next.as_bytes()[0] as char).is_whitespace() {
                continue;
            }

            return Some((pos, next));
        }
    }
}

impl<'a, T, O> Iterator for LexerIterator<'a, T, O>
where
    T: Extract<'a, O>,
{
    type Item = Result<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.skip_whitespace {
            self.next_non_whitespace()
        } else {
            self.iter.next_item()
        };

        let (pos, next) = match ret {
            Some(ret) => ret,
            None => return None,
        };

        self.iter.child(|child| T::extract(next, pos, child)).into()
    }
}
