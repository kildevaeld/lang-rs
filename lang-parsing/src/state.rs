use super::{error::Error, parse::Parse, peek::Peek, reader::TokenReader};
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

    pub fn input(&self) -> &'a str {
        self.input
    }

    pub fn is_empty(&self) -> bool {
        self.current >= self.stream.len()
    }

    pub fn parse<P>(&mut self) -> Result<P, Error>
    where
        P: Parse<'a, T>,
    {
        let mut child_idx = self.current;

        match P::parse(TokenReader {
            input: self.input,
            tokens: &self.stream,
            current: &mut child_idx,
        }) {
            Ok(ret) => {
                self.current = child_idx;
                Ok(ret)
            }
            err => err,
        }
    }

    pub fn peek<P: Peek<'a, T>>(&mut self) -> bool {
        self.reader().peek::<P>()
    }

    pub fn peek2<P: Peek<'a, T>>(&mut self) -> bool {
        self.reader().peek2::<P>()
    }

    pub fn peek3<P: Peek<'a, T>>(&mut self) -> bool {
        self.reader().peek_offset::<P>(2)
    }

    pub fn peek_while<P: Peek<'a, T>, N: Peek<'a, T>>(&mut self) -> bool {
        self.reader().peek_while::<P, N>()
    }

    fn reader<'b>(&'b mut self) -> TokenReader<'a, 'b, T> {
        TokenReader {
            input: self.input,
            tokens: &self.stream,
            current: &mut self.current,
        }
    }
}
