use core::fmt;
use gc_arena::{make_arena, ArenaParameters, Collect, MutationContext};

use crate::{
    error::Result,
    thread::{Thread, ThreadMode},
    Callable, Value,
};

#[derive(Debug, Collect)]
#[collect(no_drop)]
pub struct Root<'gc> {
    main_thread: Thread<'gc>,
}

make_arena!(VmArena, Root);

pub struct Vm {
    arena: VmArena,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            arena: VmArena::new(ArenaParameters::default(), |mc| Root {
                main_thread: Thread::new(mc),
            }),
        }
    }

    pub fn mutate<F, T>(&mut self, func: F) -> T
    where
        F: for<'gc> FnOnce(Context<'gc, '_>) -> T,
    {
        self.arena.mutate(|mc, root| {
            func(Context {
                root: &root.main_thread,
                mc,
            })
        })
    }

    pub fn total_allocated(&self) -> usize {
        self.arena.total_allocated()
    }
}

pub struct Context<'gc, 'a> {
    pub root: &'a Thread<'gc>,
    pub mc: MutationContext<'gc, 'a>,
}

impl<'gc, 'a> fmt::Debug for Context<'gc, 'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context").field("root", self.root).finish()
    }
}

impl<'gc, 'a> Context<'gc, 'a> {
    pub fn call(&self, func: Callable<'gc>, args: &[Value<'gc>]) -> Result<Value<'gc>> {
        self.root.start(self.mc, func, args)?;

        while self.root.mode() == ThreadMode::Running {
            self.root.step(self.mc)?;
        }

        let result = self.root.take_result(self.mc);

        Ok(result.unwrap_or(Value::Nil))
    }
}

impl<'gc, 'a> core::ops::Deref for Context<'gc, 'a> {
    type Target = MutationContext<'gc, 'a>;
    fn deref(&self) -> &Self::Target {
        &self.mc
    }
}
