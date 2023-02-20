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
