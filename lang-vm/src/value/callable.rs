use crate::heap::Box;
use crate::Closure;
use gc_arena::{Collect, Gc};

use crate::native::NativeFunc;

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Callable<'gc> {
    Native(Gc<'gc, Box<dyn NativeFunc<'gc>>>),
    Closure(Closure<'gc>),
}

impl<'gc> Callable<'gc> {
    pub fn as_closure(&self) -> Option<Closure<'gc>> {
        match self {
            Callable::Closure(closure) => Some(*closure),
            _ => None,
        }
    }
}
