use gc_arena::Collect;

use crate::String;

use super::chunk::Chunk;

#[derive(Debug, Clone, Collect)]
#[collect(no_drop)]
pub struct Function<'gc> {
    name: Option<String<'gc>>,
    chunk: Chunk<'gc>,
    arity: u8,
}

impl<'gc> Function<'gc> {
    pub fn new(chunk: Chunk<'gc>, arity: u8) -> Function<'gc> {
        Function {
            chunk,
            name: None,
            arity,
        }
    }

    pub fn chunk(&self) -> &Chunk<'gc> {
        &self.chunk
    }

    pub fn arity(&self) -> u8 {
        self.arity
    }
}
