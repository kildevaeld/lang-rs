use lang_lexing::{
    tokens::{Comment, Whitespace},
    Span, TokenRef, WithSpan,
};

use crate::{Cursor, Parse, Peek};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ws<T>(T);

impl<T> Ws<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> core::ops::Deref for Ws<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Ws<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T, TOKEN> Peek<'a, TOKEN> for Ws<T>
where
    T: Peek<'a, TOKEN>,
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>>,
{
    fn peek(cursor: &mut Cursor<'a, '_, TOKEN>) -> bool {
        if !cursor.peek::<Whitespace>() {
            return false;
        }

        let mut i = 1;
        while cursor.peek_offset::<Whitespace>(i) {
            i += 1;
        }

        T::peek(&mut cursor.offset(i as isize).expect("peek"))
    }
}

impl<'a, T, TOKEN> Parse<'a, TOKEN> for Ws<T>
where
    T: Parse<'a, TOKEN>,
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    fn parse(state: &mut crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
        while state.peek::<Whitespace>() {
            state.eat::<Whitespace>()?;
        }

        let ws = Ws(T::parse(state)?);

        if !state.peek::<Whitespace>() {
            return Err(state.error("whitespace"));
        }

        Ok(ws)
    }
}

impl<T> WithSpan for Ws<T>
where
    T: WithSpan,
{
    fn span(&self) -> lang_lexing::Span {
        self.0.span()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nl<'a>(Whitespace<'a>);

impl<'a, TOKEN> Peek<'a, TOKEN> for Nl<'a>
where
    TOKEN: TokenRef<Whitespace<'a>>,
{
    fn peek(cursor: &mut Cursor<'a, '_, TOKEN>) -> bool {
        cursor.peek::<Whitespace>()
    }
}

impl<'a, TOKEN> Parse<'a, TOKEN> for Nl<'a>
where
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    fn parse(state: &mut crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
        let ws = state.parse::<Whitespace>()?;

        if !ws.lexeme.contains('\n') {
            return Err(state.error("newline"));
        }

        Ok(Nl(ws))
    }
}

impl<'a> WithSpan for Nl<'a> {
    fn span(&self) -> lang_lexing::Span {
        self.0.span()
    }
}
