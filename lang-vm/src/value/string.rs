use core::hash::Hash;

use crate::heap::{Box, ToString};
use gc_arena::{Collect, Gc, MutationContext};

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
enum StringKind<'gc> {
    Static(&'static str),
    Heap(Gc<'gc, Box<str>>),
}

impl<'gc> StringKind<'gc> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Heap(b) => &*b,
            Self::Static(s) => s,
        }
    }
}

impl<'gc> PartialEq for StringKind<'gc> {
    fn eq(&self, other: &Self) -> bool {
        use StringKind::*;
        match (self, other) {
            (Static(a), Static(b)) => a == b,
            (Heap(a), Heap(b)) => {
                //
                if Gc::ptr_eq(*a, *b) {
                    true
                } else {
                    **a == **b
                }
            }
            (Static(a), Heap(b)) => *a == &***b,
            (Heap(a), Static(b)) => &***a == *b,
        }
    }
}

impl<'gc> Eq for StringKind<'gc> {}

impl<'gc> Hash for StringKind<'gc> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<'gc> PartialOrd for StringKind<'gc> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<'gc> Ord for StringKind<'gc> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

#[derive(Debug, Clone, Copy, Collect, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[collect(no_drop)]
pub struct String<'gc>(StringKind<'gc>);

impl<'gc> String<'gc> {
    pub fn new_static(data: &'static str) -> String<'gc> {
        String(StringKind::Static(data))
    }
    pub fn new(mc: MutationContext<'gc, '_>, data: &str) -> String<'gc> {
        let kind = StringKind::Heap(Gc::allocate(mc, data.to_string().into_boxed_str()));
        String(kind)
    }
}

impl<'gc> String<'gc> {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'gc> core::fmt::Display for String<'gc> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl<'gc> AsRef<str> for String<'gc> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
