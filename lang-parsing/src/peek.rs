// use super::cursor::Cursor;

use crate::TokenReader;

pub trait Peek<'a, T> {
    fn peek(cursor: &mut TokenReader<'a, '_, T>) -> bool;
}
