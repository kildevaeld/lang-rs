use gc_arena::MutationContext;

use super::execute::execute;
use crate::bytecode::Chunk;
use crate::error::Result;
use crate::heap::HashMap;
use crate::{String, Value};
use core::marker::PhantomData;

pub trait ModuleState<'gc> {}

pub struct Module<'gc, S> {
    pub(crate) state: S,
    pub(crate) _p: PhantomData<&'gc ()>,
}

pub struct ByteCodeState<'gc> {
    pub(crate) chunk: Chunk<'gc>,
}

impl<'gc> ModuleState<'gc> for ByteCodeState<'gc> {}

impl<'gc> Module<'gc, ByteCodeState<'gc>> {
    pub fn load(self, mc: MutationContext<'gc, '_>) -> Result<Module<'gc, LoadedState<'gc>>> {
        execute(&self)
    }
}

pub struct LoadedState<'gc> {
    pub(crate) exports: HashMap<String<'gc>, Value<'gc>>,
}

impl<'gc> ModuleState<'gc> for LoadedState<'gc> {}
