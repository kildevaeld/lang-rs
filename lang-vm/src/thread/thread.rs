use std::println;

use gc_arena::{Collect, GcCell, MutationContext};

use crate::{
    call_frame::CallFrame,
    error::Result,
    thread::bytecode_frame::BytecodeFrame,
    value::{Callable, Value},
};

use super::{
    execute::execute,
    state::{ThreadMode, ThreadState},
    utils::call_callable,
};

#[derive(Clone, Copy, Debug, Collect)]
#[collect(no_drop)]
pub struct Thread<'gc>(GcCell<'gc, ThreadState<'gc>>);

impl<'gc> Thread<'gc> {
    pub fn new(mc: MutationContext<'gc, '_>) -> Thread<'gc> {
        Thread(GcCell::allocate(mc, ThreadState::new(mc)))
    }

    pub fn mode(&self) -> ThreadMode {
        if let Ok(state) = self.0.try_read() {
            state.get_mode()
        } else {
            ThreadMode::Running
        }
    }

    pub fn dump_stack(&self) {
        println!("stack {:#?}", self.0.read().stack);
    }

    pub fn start(
        &self,
        mc: MutationContext<'gc, '_>,
        function: Callable<'gc>,
        args: &[Value<'gc>],
    ) -> Result<()> {
        let mut thread = self.0.write(mc);
        thread.check_mode(ThreadMode::Stopped)?;
        call_callable(*self, &mut thread, function, args)
    }

    pub fn take_result(&self, mc: MutationContext<'gc, '_>) -> Option<Value<'gc>> {
        let mut state = self.0.write(mc);
        state.result.take()
    }

    pub fn step(&self, mc: MutationContext<'gc, '_>) -> Result<()> {
        let mut state = self.0.write(mc);

        state.check_mode(ThreadMode::Running)?;

        match state.call_stack.last_mut() {
            Some(CallFrame::ByteCode { .. }) => {
                const VM_GRANULARITY: u32 = 256;
                let mut instructions = VM_GRANULARITY;

                loop {
                    let frame = BytecodeFrame {
                        thread: *self,
                        state: &mut state,
                    };

                    instructions = execute(frame, mc, instructions)?;

                    if let Some(CallFrame::ByteCode { .. }) = state.call_stack.last() {
                        if instructions == 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            _ => {
                panic!("invalid state")
            }
        }

        Ok(())
    }
}
