use gc_arena::Collect;

use crate::call_frame::CallFrame;
use crate::heap::Vec;

#[derive(Debug, Collect, Default)]
#[collect(no_drop)]
pub struct CallStack<'gc> {
    stack: Vec<CallFrame<'gc>>,
}

impl<'gc> CallStack<'gc> {
    pub fn last_mut(&mut self) -> Option<&mut CallFrame<'gc>> {
        self.stack.last_mut()
    }

    pub fn last(&self) -> Option<&CallFrame<'gc>> {
        self.stack.last()
    }

    pub fn push(&mut self, frame: CallFrame<'gc>) {
        self.stack.push(frame);
    }

    /*pub fn len(&self) -> usize {
        self.stack.len()
    }*/

    pub fn pop(&mut self) -> Option<CallFrame<'gc>> {
        self.stack.pop()
    }
}
