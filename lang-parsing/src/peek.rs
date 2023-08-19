// use super::cursor::Cursor;

use crate::TokenReader;

pub trait Peek<'a, T> {
    fn peek(cursor: TokenReader<'a, '_, T>) -> bool;
}
