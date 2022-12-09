use gc_arena::{Collect, Gc, MutationContext};

use crate::bytecode::{Chunk, Function};

#[derive(Debug, Collect)]
#[collect(no_drop)]
struct ClosureInner<'gc> {
    pub func: Function<'gc>,
    pub upvalue_count: u8,
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct Closure<'gc>(Gc<'gc, ClosureInner<'gc>>);

impl<'gc> Closure<'gc> {
    pub fn new(
        mc: MutationContext<'gc, '_>,
        func: Function<'gc>,
        upvalue_count: u8,
    ) -> Closure<'gc> {
        Closure(Gc::allocate(
            mc,
            ClosureInner {
                func,
                upvalue_count,
            },
        ))
    }

    pub fn arity(&self) -> u8 {
        self.0.func.arity()
    }

    pub fn chunk(&self) -> &Chunk<'gc> {
        self.0.func.chunk()
    }
}
