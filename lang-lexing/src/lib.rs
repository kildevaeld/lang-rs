#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod cursor;
mod error;
mod extract;
mod lexer;
mod span;
pub mod utils;

pub use self::{
    cursor::Cursor,
    error::{Error, Result},
    extract::Extract,
    lexer::*,
    span::*,
};

pub trait LexerFactory<'a>: Sized {
    type Extract: Extract<'a, Self>;
    fn create_lexer(input: &'a str) -> Lexer<'a, Self::Extract, Self>;
}

pub trait TokenRef<T> {
    fn value(&self) -> Option<&T>;
}
