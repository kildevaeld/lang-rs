use lang_lexing::{
    tokens::{Comment, Whitespace},
    TokenRef, WithSpan,
};

use crate::{Parse, Peek, TokenReader};

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
    fn peek(cursor: TokenReader<'a, '_, TOKEN>) -> bool {
        if !cursor.peek::<Whitespace>() {
            return false;
        }

        let mut i = 1;
        while cursor.peek_offset::<Whitespace>(i) {
            i += 1;
        }

        cursor.peek_offset::<T>(i)
    }
}

impl<'a, T, TOKEN> Parse<'a, TOKEN> for Ws<T>
where
    T: Parse<'a, TOKEN>,
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    fn parse(mut state: crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
        if !state.peek::<Whitespace>() {
            return Err(state.error("whitespace"));
        }

        while state.peek::<Whitespace>() {
            state.eat::<Whitespace>()?;
        }

        let ws = Ws(state.parse::<T>()?);

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
    fn peek(cursor: TokenReader<'a, '_, TOKEN>) -> bool {
        cursor.peek::<Whitespace>()
    }
}

impl<'a, TOKEN> Parse<'a, TOKEN> for Nl<'a>
where
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    fn parse(mut state: crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoWs<T>(T);

impl<T> NoWs<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> core::ops::Deref for NoWs<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for NoWs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T, TOKEN> Peek<'a, TOKEN> for NoWs<T>
where
    T: Peek<'a, TOKEN>,
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>>,
{
    fn peek(cursor: TokenReader<'a, '_, TOKEN>) -> bool {
        !cursor.peek::<Whitespace>() && cursor.peek::<T>()
    }
}

impl<'a, T, TOKEN> Parse<'a, TOKEN> for NoWs<T>
where
    T: Parse<'a, TOKEN>,
    TOKEN: TokenRef<Whitespace<'a>> + TokenRef<Comment<'a>> + WithSpan,
{
    fn parse(mut state: crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
        if state.peek::<Whitespace>() {
            return Err(state.error("whitespace"));
        }

        let nows = NoWs(state.parse()?);

        if state.peek::<Whitespace>() {
            return Err(state.error("whitespace"));
        }

        Ok(nows)
    }
}

impl<T> WithSpan for NoWs<T>
where
    T: WithSpan,
{
    fn span(&self) -> lang_lexing::Span {
        self.0.span()
    }
}
