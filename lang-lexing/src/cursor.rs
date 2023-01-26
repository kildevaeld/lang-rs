use alloc::{fmt, vec::Vec};
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

use crate::Error;

struct State<'a> {
    input: &'a str,
    iter: Option<GraphemeIndices<'a>>,
    buffer: Vec<(usize, &'a str)>,
}

impl<'a> State<'a> {
    fn inner_next(&mut self) -> Option<(usize, &'a str)> {
        match &mut self.iter {
            Some(i) => match i.next() {
                Some(ret) => {
                    self.buffer.push(ret);
                    Some(ret)
                }
                None => {
                    self.iter = None;
                    None
                }
            },
            _ => None,
        }
    }

    fn ensure_index(&mut self, idx: usize) -> Option<usize> {
        while idx >= self.buffer.len() {
            if self.inner_next().is_none() {
                break;
            }
        }

        let len = self.buffer.len();
        if len == 0 {
            None
        } else {
            Some(idx.min(len - 1))
        }
    }

    fn get(&mut self, idx: usize) -> Option<(usize, &'a str)> {
        self.ensure_index(idx);
        self.try_get(idx)
    }

    fn try_get(&self, idx: usize) -> Option<(usize, &'a str)> {
        self.buffer.get(idx).copied()
    }

    // fn is_finished(&self) -> bool {
    //     self.iter.is_none()
    // }

    // fn len(&mut self) -> usize {
    //     while let Some(next) = self.inner_next() {
    //         let _ = next;
    //     }

    //     self.buffer.len()
    // }
}

struct StateReader<'a, 'b> {
    state: &'b mut State<'a>,
    current_idx: &'b mut Option<usize>,
}

impl<'a, 'b> StateReader<'a, 'b> {
    fn new(state: &'b mut State<'a>, current_idx: &'b mut Option<usize>) -> StateReader<'a, 'b> {
        StateReader { state, current_idx }
    }

    fn current(&self) -> Option<(usize, &'a str)> {
        self.current_idx.and_then(|idx| self.state.try_get(idx))
    }

    fn peekn(&mut self, idx: usize) -> Option<(usize, &'a str)> {
        let real_idx = self.current_idx.map(|m| m + 1).unwrap_or(0) + idx;
        self.state.get(real_idx)
    }

    fn prev_item(&mut self) -> Option<(usize, &'a str)> {
        let idx = match self.current_idx {
            Some(idx) => *idx,
            None => return None,
        };

        if idx == 0 {
            *self.current_idx = None;
            return None;
        }

        *self.current_idx = Some(idx - 1);

        self.current()
    }

    fn next_item(&mut self) -> Option<(usize, &'a str)> {
        let idx = self.current_idx.map(|m| m + 1).unwrap_or(0);
        let ret = self.state.get(idx)?;

        *self.current_idx = Some(idx);

        Some(ret)
    }

    fn goto_index(&mut self, idx: usize) -> bool {
        if Some(idx) == *self.current_idx {
            return true;
        }

        if let Some(new_idx) = self.state.ensure_index(idx) {
            if idx != new_idx {
                false
            } else {
                *self.current_idx = Some(new_idx);
                true
            }
        } else {
            false
        }
    }
}

pub struct Cursor<'a> {
    state: State<'a>,
    current_idx: Option<usize>,
}

impl<'a> fmt::Debug for Cursor<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Iter")
            .field("input", &self.state.input)
            .field("current_idx", &self.current_idx)
            .finish()
    }
}

macro_rules! reader {
    ($this: expr) => {
        StateReader::new(&mut $this.state, &mut $this.current_idx)
    };
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str, extended: bool) -> Cursor<'a> {
        Cursor {
            state: State {
                input,
                iter: input.grapheme_indices(extended).into(),
                buffer: Vec::with_capacity(input.len()),
            },
            current_idx: None,
        }
    }

    pub fn input(&self) -> &'a str {
        self.state.input
    }

    pub fn current(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).current()
    }

    pub fn peek(&mut self) -> Option<(usize, &'a str)> {
        self.peekn(0)
    }

    pub fn peekn(&mut self, idx: usize) -> Option<(usize, &'a str)> {
        reader!(self).peekn(idx)
    }

    pub fn prev_item(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).prev_item()
    }

    pub fn next_item(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).next_item()
    }

    pub fn goto_index(&mut self, idx: usize) -> bool {
        reader!(self).goto_index(idx)
    }

    // pub fn len(&mut self) -> usize {
    //     self.state.len()
    // }

    pub fn child<F, R>(&mut self, mut func: F) -> Result<R, Error<'a>>
    where
        F: FnMut(&mut ChildCursor<'a, '_>) -> Result<R, Error<'a>>,
    {
        let mut child = ChildCursor {
            current_idx: self.current_idx,
            state: &mut self.state,
        };

        match func(&mut child) {
            Ok(ret) => {
                self.current_idx = child.current_idx;
                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }
}

impl<'a> Iterator for Cursor<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

pub struct ChildCursor<'a, 'b> {
    current_idx: Option<usize>,
    state: &'b mut State<'a>,
}

impl<'a, 'b> ChildCursor<'a, 'b> {
    pub fn input(&self) -> &'a str {
        self.state.input
    }

    pub fn current(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).current()
    }

    pub fn peek(&mut self) -> Option<(usize, &'a str)> {
        self.peekn(0)
    }

    pub fn peekn(&mut self, idx: usize) -> Option<(usize, &'a str)> {
        reader!(self).peekn(idx)
    }

    pub fn prev_item(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).prev_item()
    }

    pub fn next_item(&mut self) -> Option<(usize, &'a str)> {
        reader!(self).next_item()
    }

    pub fn goto_index(&mut self, idx: usize) -> bool {
        reader!(self).goto_index(idx)
    }

    // pub fn len(&mut self) -> usize {
    //     self.state.len()
    // }

    pub fn child<F, R>(&mut self, mut func: F) -> Result<R, Error<'a>>
    where
        F: FnMut(&mut ChildCursor<'a, '_>) -> Result<R, Error<'a>>,
    {
        let mut child = ChildCursor {
            current_idx: self.current_idx,
            state: self.state,
        };

        match func(&mut child) {
            Ok(ret) => {
                self.current_idx = child.current_idx;
                Ok(ret)
            }
            Err(err) => Err(err),
        }
    }
}

impl<'a, 'b> Iterator for ChildCursor<'a, 'b> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::vec;

    #[test]
    fn next_item() {
        let mut iter = Cursor::new("Hello, World", true);

        assert_eq!(iter.next_item(), Some((0, "H")));
        assert_eq!(iter.next_item(), Some((1, "e")));
        assert_eq!(iter.next_item(), Some((2, "l")));
        assert_eq!(iter.next_item(), Some((3, "l")));
    }

    #[test]
    fn prev_item() {
        let mut iter = Cursor::new("Hello, World", true);

        assert_eq!(iter.prev_item(), None);
        assert_eq!(iter.next_item(), Some((0, "H")));
        assert_eq!(iter.prev_item(), None);

        assert_eq!(iter.next_item(), Some((0, "H")));
        assert_eq!(iter.next_item(), Some((1, "e")));
        assert_eq!(iter.next_item(), Some((2, "l")));
    }

    #[test]
    fn ensure_index() {
        let mut iter = Cursor::new("Hello, World", true);

        iter.state.ensure_index(0);

        assert_eq!(iter.state.buffer, vec![(0, "H")]);

        iter.state.ensure_index(4);

        assert_eq!(
            iter.state.buffer,
            vec![(0, "H"), (1, "e"), (2, "l"), (3, "l"), (4, "o")]
        );

        iter.state.ensure_index(1);

        assert_eq!(iter.current_idx, None);
    }

    #[test]
    fn peek() {
        let mut iter = Cursor::new("Hello, World", true);

        assert_eq!(iter.peek(), Some((0, "H")));
        assert_eq!(iter.peekn(1), Some((1, "e")));
        assert_eq!(iter.peekn(4), Some((4, "o")));

        assert_eq!(iter.current_idx, None);

        assert_eq!(iter.next_item(), Some((0, "H")));

        assert_eq!(iter.peek(), Some((1, "e")));

        iter.next_item();

        assert_eq!(iter.peek(), Some((2, "l")));
        assert_eq!(iter.peekn(2), Some((4, "o")));

        iter.prev_item();

        assert_eq!(iter.peek(), Some((1, "e")))
    }

    // #[test]
    // fn len() {
    //     let mut iter = Cursor::new("Hello", true);

    //     // assert_eq!(iter.len(), 5);
    //     assert_eq!(iter.current_idx, None);
    //     assert_eq!(
    //         iter.state.buffer,
    //         vec![(0, "H"), (1, "e"), (2, "l"), (3, "l"), (4, "o")]
    //     );
    // }

    #[test]
    fn child() {
        let mut iter = Cursor::new("Hello", true);

        let ret = iter.child(|child| {
            let _ = child.next_item();
            Result::<(), _>::Err(Error::new(0, "Hello"))
        });

        assert_eq!(ret, Err(Error::new(0, "Hello")));
        assert_eq!(iter.state.buffer, vec![(0, "H")]);
        assert_eq!(iter.current_idx, None);
    }

    #[test]
    fn goto_index() {
        let mut iter = Cursor::new("Hello", true);

        iter.goto_index(3);
        assert_eq!(iter.current(), Some((3, "l")));
        iter.goto_index(1);
        assert_eq!(iter.current(), Some((1, "e")));
        iter.goto_index(4);
        assert_eq!(iter.current(), Some((4, "o")));
    }
}
