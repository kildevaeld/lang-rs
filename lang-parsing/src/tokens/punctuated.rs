use crate::{Error, Parse, Peek, TokenReader};
use alloc::vec::Vec;

#[derive(Debug, Clone)]
enum Entry<T, P> {
    Item(T),
    Punct(P),
}

#[derive(Debug, Clone)]
pub struct Punctuated<T, P> {
    items: Vec<Entry<T, P>>,
}

impl<'a, T, P, TOKEN> Peek<'a, TOKEN> for Punctuated<T, P>
where
    T: Peek<'a, TOKEN>,
{
    fn peek(cursor: &mut crate::Cursor<'a, '_, TOKEN>) -> bool {
        T::peek(cursor)
    }
}

impl<'a, T, P, TOKEN> Parse<'a, TOKEN> for Punctuated<T, P>
where
    T: Peek<'a, TOKEN> + Parse<'a, TOKEN>,
    P: Peek<'a, TOKEN> + Parse<'a, TOKEN>,
{
    fn parse(state: &mut TokenReader<'a, '_, TOKEN>) -> Result<Self, Error> {
        let mut items = alloc::vec::Vec::new();

        loop {
            if !state.peek::<T>() {
                break;
            }

            items.push(Entry::Item(state.parse()?));

            if !state.peek::<P>() {
                break;
            }

            items.push(Entry::Punct(state.parse()?));
        }

        Ok(Punctuated { items })
    }
}

impl<T, P> Punctuated<T, P> {
    pub fn iter(&self) -> PunctuatedIter<'_, T, P> {
        PunctuatedIter {
            iter: self.items.iter(),
        }
    }
}

impl<T, P> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = PunctuatedIntoIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        PunctuatedIntoIter {
            iter: self.items.into_iter(),
        }
    }
}

pub struct PunctuatedIntoIter<T, P> {
    iter: alloc::vec::IntoIter<Entry<T, P>>,
}

impl<T, P> Iterator for PunctuatedIntoIter<T, P> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.iter.next() {
                Some(next) => next,
                None => return None,
            };

            match next {
                Entry::Item(item) => return Some(item),
                Entry::Punct(_) => {}
            }
        }
    }
}

pub struct PunctuatedIter<'a, T, P> {
    iter: core::slice::Iter<'a, Entry<T, P>>,
}

impl<'a, T, P> Iterator for PunctuatedIter<'a, T, P> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.iter.next() {
                Some(next) => next,
                None => return None,
            };

            match next {
                Entry::Item(item) => return Some(item),
                Entry::Punct(_) => {}
            }
        }
    }
}
