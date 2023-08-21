#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(test)]
extern crate std;

mod cursor;
mod error;
mod extract;
mod lexer;
mod span;
mod string_ext;

#[cfg(feature = "tokens")]
pub mod tokens;

pub use self::{
    cursor::{ChildCursor, Cursor},
    error::{Error, Result},
    extract::Extract,
    lexer::{Lexer, LexerIterator},
    span::{Span, WithSpan},
    string_ext::StringExt,
};

pub trait LexerFactory<'a, O>: Sized {
    type Extract: Extract<'a, O>;
    fn create_lexer(input: &'a str) -> Lexer<'a, Self::Extract, O>;
}

pub trait TokenRef<T> {
    fn value(&self) -> Option<&T>;
}
