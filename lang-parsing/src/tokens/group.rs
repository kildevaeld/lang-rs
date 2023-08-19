use lang_lexing::WithSpan;

use crate::{Parse, Peek};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Group<O, T, C> {
    pub open_token: O,
    pub value: T,
    pub close_token: C,
}

impl<'a, O, T, C, TOKEN> Peek<'a, TOKEN> for Group<O, T, C>
where
    O: Peek<'a, TOKEN>,
{
    fn peek(cursor: crate::TokenReader<'a, '_, TOKEN>) -> bool {
        O::peek(cursor)
    }
}

impl<'a, O, T, C, TOKEN> Parse<'a, TOKEN> for Group<O, T, C>
where
    O: Parse<'a, TOKEN>,
    T: Parse<'a, TOKEN>,
    C: Parse<'a, TOKEN>,
{
    fn parse(mut state: crate::TokenReader<'a, '_, TOKEN>) -> Result<Self, crate::Error> {
        Ok(Group {
            open_token: state.parse()?,
            value: state.parse()?,
            close_token: state.parse()?,
        })
    }
}

impl<O, T, C> WithSpan for Group<O, T, C>
where
    O: WithSpan,
    C: WithSpan,
{
    fn span(&self) -> lang_lexing::Span {
        self.open_token.span() + self.close_token.span()
    }
}
