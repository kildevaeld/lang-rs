use crate::heap::Box;
use crate::native::NativeFunc;
use gc_arena::{Collect, Gc};

#[derive(Debug, Collect)]
#[collect(no_drop)]
pub enum CallFrame<'gc> {
    ByteCode {
        /// Current instruction pointer
        ip: usize,
        ///  
        base: usize,
        /// Index of the closure in the stack
        bottom: usize,
    },
    Callback {
        callback: Gc<'gc, Box<dyn NativeFunc<'gc>>>,
    },
}
