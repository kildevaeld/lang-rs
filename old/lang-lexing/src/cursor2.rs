use crate::Span;
use core::{iter::Peekable, str::CharIndices};
use unicode_segmentation::GraphemeIndices;

pub struct Cursor<'a> {
    input: &'a str,
    chars: Peekable<GraphemeIndices<'a>>,
    current: usize,
}

impl<'a> Cursor<'a> {}

impl<'a> Iterator for Cursor<'a> {
    type Item = (Span, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.chars.next();
        }
        todo!()
    }
}
