use super::{error::Error, parse::Parse, reader::TokenReader, token::Peek};
use alloc::vec::Vec;

use lang_lexing::{Error as LexerError, LexerFactory};

pub struct Parser<'a, T> {
    input: &'a str,
    stream: Vec<T>,
    current: usize,
}

impl<'a, T> Parser<'a, T> {
    pub fn new(input: &'a str) -> Result<Parser<'a, T>, LexerError<'a>>
    where
        T: LexerFactory<'a, T>,
    {
        let lexer = T::create_lexer(input);
        let stream = lexer.tokenize();

        Parser::from_tokens(input, stream)
    }

    pub fn from_tokens<I>(input: &'a str, iter: I) -> Result<Parser<'a, T>, LexerError>
    where
        I: Iterator<Item = Result<T, LexerError<'a>>>,
    {
        let tokens = iter.collect::<Result<_, _>>()?;

        let state = Parser {
            input,
            stream: tokens,
            current: 0,
        };

        Ok(state)
    }

    pub(crate) fn reader<'b>(&'b mut self) -> TokenReader<'a, 'b, T> {
        TokenReader {
            input: self.input,
            tokens: &self.stream,
            current: self.current,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.current >= self.stream.len()
    }

    pub fn parse<P>(&mut self) -> Result<P, Error>
    where
        P: Parse<'a, T>,
    {
        let mut cursor = TokenReader {
            input: self.input,
            tokens: &self.stream,
            current: self.current,
        };

        match P::parse(&mut cursor) {
            Ok(ret) => {
                self.current = cursor.current;
                Ok(ret)
            }
            err => err,
        }
    }

    pub fn peek<P: Peek<'a, T>>(&mut self, token: P) -> bool {
        self.reader().peek(token)
    }

    pub fn peek2<P: Peek<'a, T>>(&mut self, token: P) -> bool {
        self.reader().peek2(token)
    }

    pub fn peek3<P: Peek<'a, T>>(&mut self, token: P) -> bool {
        self.reader().peek_offset(token, 2)
    }
}
