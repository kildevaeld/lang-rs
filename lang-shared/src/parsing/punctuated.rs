use alloc::vec::Vec;
use lang_parsing::{Error, Parse, TokenReader};

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
}

impl<T, P> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = PunctuatedIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        PunctuatedIter {
            iter: self.items.into_iter(),
        }
    }
}

pub struct PunctuatedIter<T, P> {
    iter: alloc::vec::IntoIter<Entry<T, P>>,
}

impl<T, P> Iterator for PunctuatedIter<T, P> {
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
