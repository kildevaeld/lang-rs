use core::fmt;

pub trait WithSpan {
    fn span(&self) -> Span;
}

#[cfg(feature = "either")]
impl<L, R> WithSpan for either::Either<L, R>
where
    L: WithSpan,
    R: WithSpan,
{
    fn span(&self) -> Span {
        match self {
            either::Either::Left(m) => m.span(),
            either::Either::Right(m) => m.span(),
        }
    }
}

impl<T> WithSpan for Option<T>
where
    T: WithSpan,
{
    fn span(&self) -> Span {
        match self {
            Some(s) => s.span(),
            None => Span::new(0, 0),
        }
    }
}

impl<T> WithSpan for alloc::vec::Vec<T>
where
    T: WithSpan,
{
    fn span(&self) -> Span {
        if self.is_empty() {
            Span::default()
        } else {
            let (first, last) = match (self.first(), self.last()) {
                (Some(first), Some(last)) => (first.span(), last.span()),
                _ => return Span::default(),
            };

            first + last
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub fn is_valid(&self) -> bool {
        self.start < self.end
    }

    pub fn slice<'a>(&self, input: &'a str) -> Option<&'a str> {
        if !self.is_valid() || self.end > input.len() {
            None
        } else {
            Some(&input[self.start..self.end])
        }
    }
}

impl WithSpan for Span {
    fn span(&self) -> Span {
        *self
    }
}

impl<'a> From<(usize, &'a str)> for Span {
    fn from((pos, s): (usize, &'a str)) -> Self {
        Span::new(pos, pos + s.len())
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.start, self.end)
    }
}

impl core::ops::Add for Span {
    type Output = Span;
    fn add(mut self, rhs: Self) -> Self::Output {
        if !rhs.is_valid() {
            return self;
        } else if !self.is_valid() {
            return rhs;
        }

        if rhs.start < self.start {
            self.start = rhs.start;
        }
        if rhs.end > self.end {
            self.end = rhs.end;
        }

        self
    }
}

impl core::ops::AddAssign for Span {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

macro_rules! withspan_impl {
    ($first: ident) => {

        impl<$first: WithSpan> WithSpan for ($first, ) {
            fn span(&self) -> Span {
                self.0.span()
            }
        }


    };
    ($first: ident $($rest:ident)*) => {
        withspan_impl!($($rest)*);

        #[allow(non_snake_case)]
        impl<$first: WithSpan, $($rest: WithSpan),*> WithSpan for ($first, $($rest),*) {
            fn span(&self) -> Span {
                let ($first, $($rest),*) = self;
                $first.span()  $(+ $rest.span())*
            }
        }
    };
}

withspan_impl!(T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16);

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! span {
        ($start:expr, $end: expr) => {
            Span::new($start, $end)
        };
    }

    #[test]
    fn test_invalid() {
        assert!(!Span::default().is_valid());
        assert!(!Span::new(0, 0).is_valid());
        assert!(!Span::new(2, 1).is_valid());
    }

    #[test]
    fn test_add() {
        assert_eq!(span!(0, 36), span!(0, 30) + span!(20, 36));
        assert_eq!(span!(0, 40), span!(0, 10) + span!(30, 40));
        assert_eq!(span!(10, 30), span!(15, 17) + span!(10, 30));
        assert_eq!(span!(0, 1), span!(0, 1) + span!(0, 0));
        assert_eq!(span!(0, 1), span!(0, 0) + span!(0, 1));
    }
}
