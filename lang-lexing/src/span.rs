use core::fmt;

pub trait WithSpan {
    fn span(&self) -> &Span;
}

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

impl<'a> fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.start, self.end)
    }
}

impl core::ops::Add for Span {
    type Output = Span;
    fn add(mut self, rhs: Self) -> Self::Output {
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
