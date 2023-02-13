use super::cursor::Cursor;

// pub trait Peek<'a, T> {
//     type Token: Token<'a, T>;
// }

// impl<'a, F, U, T> Peek<'a, U> for F
// where
//     F: Fn(Span) -> T,
//     T: Token<'a, U>,
// {
//     type Token = T;
// }

pub trait Peek<'a, T> {
    fn peek(cursor: &mut Cursor<'a, '_, T>) -> bool;
}
