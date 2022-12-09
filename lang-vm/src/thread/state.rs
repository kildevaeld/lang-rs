use crate::{
    call_frame::CallFrame, call_stack::CallStack, error::BadThreadMode, heap::Vec, stack::Stack,
    Value,
};
use gc_arena::{Collect, MutationContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadMode {
    Stopped,
    Results,
    Running,
}

#[derive(Collect, Debug)]
#[collect(no_drop)]
pub struct ThreadState<'gc> {
    pub(crate) stack: Stack<'gc>,
    pub(crate) call_stack: CallStack<'gc>,
    pub(crate) result: Option<Value<'gc>>,
    pub(crate) open_upvalues: Vec<Value<'gc>>,
}

impl<'gc> ThreadState<'gc> {
    pub fn new(mc: MutationContext<'gc, '_>) -> ThreadState<'gc> {
        ThreadState {
            stack: Stack::default(),
            call_stack: CallStack::default(),
            result: None,
            open_upvalues: Vec::default(),
        }
    }

    pub fn check_mode(&self, expected: ThreadMode) -> Result<(), BadThreadMode> {
        let found = self.get_mode();
        if found != expected {
            Err(BadThreadMode {
                expected: Some(expected),
                found,
            })
        } else {
            Ok(())
        }
    }

    pub fn get_mode(&self) -> ThreadMode {
        if self.result.is_some() {
            ThreadMode::Results
        } else {
            match self.call_stack.last() {
                None => {
                    assert!(
                        self.stack.is_empty()
                            && self.open_upvalues.is_empty()
                            && self.result.is_none(),
                    );
                    ThreadMode::Stopped
                }
                Some(frame) => match frame {
                    CallFrame::Callback { .. }
                    //| CallFrame::Continuation { .. }
                    | CallFrame::ByteCode { .. } => ThreadMode::Running,
                    _ => ThreadMode::Stopped,
                    //Frame::StartCoroutine(_) | Frame::ResumeCoroutine => ThreadMode::Suspended,
                },
            }
        }
    }
}
