use crate::{heap::HashSet, String};
use gc_arena::{Collect, GcCell, MutationContext};

#[derive(Debug, Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct StringInterner<'gc> {
    strings: GcCell<'gc, HashSet<String<'gc>>>,
}

impl<'gc> StringInterner<'gc> {
    pub fn new(mc: MutationContext<'gc, '_>) -> StringInterner<'gc> {
        StringInterner {
            strings: GcCell::allocate(mc, HashSet::default()),
        }
    }

    pub fn intern_or_get<S>(&self, mc: MutationContext<'gc, '_>, string: S) -> String<'gc>
    where
        S: AsRef<str>,
    {
        *self
            .strings
            .write(mc)
            .get_or_insert_with(string.as_ref(), |_| String::new(mc, string.as_ref()))
    }

    pub fn register(&self, mc: MutationContext<'gc, '_>, value: String<'gc>) {
        self.strings.write(mc).insert(value);
    }
}
