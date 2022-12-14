use super::{state::ThreadState, Thread};
use crate::{
    call_frame::CallFrame,
    error::Result,
    stack::StackContext,
    value::{Callable, Value},
};

pub fn call_callable<'gc>(
    thread: Thread<'gc>,
    state: &mut ThreadState<'gc>,
    callable: Callable<'gc>,
    args: &[Value<'gc>],
) -> Result<()> {
    match callable {
        Callable::Native(native) => {
            //
            let ret = native.call(args)?;

            call_return(thread, state, ret);

            Ok(())
        }
        Callable::Closure(closure) => {
            let bottom = state.stack.top();
            let base = bottom + 1;

            state.stack.resize(base + args.len());

            state.stack[bottom] = Value::Callable(Callable::Closure(closure));

            for (idx, value) in args.iter().enumerate() {
                state.stack[base + idx] = *value;
            }

            state.call_stack.push(CallFrame::ByteCode {
                ip: 0,
                base,
                bottom,
            });

            Ok(())
        }
    }
}

fn call_return<'gc>(_thread: Thread<'gc>, state: &mut ThreadState<'gc>, returns: Value<'gc>) {
    match state.call_stack.last_mut() {
        Some(CallFrame::ByteCode { .. }) => {
            panic!("bytes code not supported")
        }
        Some(CallFrame::Callback { .. }) => {}
        None => state.result = Some(returns),
    }
}
