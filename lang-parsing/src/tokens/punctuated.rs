use crate::{Error, Parse, TokenReader};
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

impl<T, P> Punctuated<T, P> {
    pub fn terminated<'a, TOKEN>(state: &mut TokenReader<'a, '_, TOKEN>) -> Result<Self, Error>
    where
        T: Parse<'a, TOKEN>,
        P: Parse<'a, TOKEN>,
    {
        let mut items = Vec::default();

        loop {
            if state.is_empty() {
                break;
            }

            let item = match state.parse::<T>() {
                Ok(ret) => ret,
                Err(_) => break,
            };

            items.push(Entry::Item(item));

            if state.is_empty() {
                break;
            }

            let punct = match state.parse::<P>() {
                Ok(ret) => ret,
                Err(_) => break,
            };

            items.push(Entry::Punct(punct));
        }

        Ok(Punctuated { items })
    }

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
