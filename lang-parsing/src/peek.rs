use super::cursor::Cursor;

pub trait Peek<'a, T> {
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool;
}
