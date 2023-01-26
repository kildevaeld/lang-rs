use core::{fmt, marker::PhantomData};
use unicode_segmentation::UnicodeSegmentation;

use crate::{cursor::Cursor, error::Result, extract::Extract, Span};

pub struct Lexer<'a, T, O> {
    cursor: Cursor<'a>,
    skip_whitespace: bool,
    _o: PhantomData<O>,
    _t: PhantomData<T>,
}

impl<'a, T, O> fmt::Debug for Lexer<'a, T, O> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
            .field("cursor", &self.cursor)
            .field("skip_whitespace", &self.skip_whitespace)
            .finish()
    }
}

impl<'a, T, O> Lexer<'a, T, O> {
    pub fn new(input: &'a str) -> Lexer<'a, T, O> {
        Lexer {
            cursor: Cursor {
                input,
                iter: input.split_word_bound_indices().peekable(),
                current_span: Span::default(),
            },
            skip_whitespace: false,
            _o: PhantomData,
            _t: PhantomData,
        }
    }

    pub fn input(&self) -> &'a str {
        self.cursor.input()
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
            cursor: self.cursor,
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
    cursor: Cursor<'a>,
    skip_whitespace: bool,
    _o: PhantomData<O>,
    _t: PhantomData<T>,
}

impl<'a, T, O> Iterator for LexerIterator<'a, T, O>
where
    T: Extract<'a, O>,
{
    type Item = Result<'a, O>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.skip_whitespace {
            self.cursor.next_non_whitespace()
        } else {
            self.cursor.next()
        };

        let (pos, next) = match ret {
            Some(ret) => ret,
            None => return None,
        };

        let ret = T::extract(next, pos, &mut self.cursor);

        Some(ret)
    }
}
