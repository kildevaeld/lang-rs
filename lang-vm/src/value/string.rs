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

#[derive(Debug, Clone, Copy, Collect)]
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

impl<'gc> PartialEq for String<'gc> {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (StringKind::Heap(l), StringKind::Heap(r)) => {
                let equal = Gc::ptr_eq(*l, *r);
                if equal {
                    return true;
                }
            }
            _ => {}
        };

        self.0.as_str() == other.0.as_str()
    }
}
