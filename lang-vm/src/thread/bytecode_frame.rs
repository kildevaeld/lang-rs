use super::{state::ThreadState, Thread};
use crate::{
    bytecode::Opcode,
    call_frame::CallFrame,
    error::Result,
    stack::{Stack, StackContext},
    Callable, Closure, Value,
};
use byteorder::ByteOrder;

pub struct BytecodeFrame<'gc, 'a> {
    pub thread: Thread<'gc>,
    pub state: &'a mut ThreadState<'gc>,
}

impl<'gc, 'a> BytecodeFrame<'gc, 'a> {
    pub fn reader<'b>(&'b mut self) -> BytecodeReader<'gc, 'b> {
        let closure = match self.state.call_stack.last() {
            Some(CallFrame::ByteCode { bottom, .. }) => {
                match self.state.stack[*bottom].as_closure() {
                    Some(closure) => closure,
                    None => panic!("not closure"),
                }
            }
            _ => panic!("not bytecode"),
        };

        match self.state.call_stack.last_mut() {
            Some(CallFrame::ByteCode { ip, bottom, .. }) => BytecodeReader {
                stack: &mut self.state.stack,
                ip,
                closure,
                bottom: *bottom,
            },
            _ => panic!(""),
        }
    }

    pub fn returns(&mut self, idx: usize) {
        match self.state.call_stack.pop() {
            Some(CallFrame::ByteCode { bottom, .. }) => {
                //let start = base + idx;
                match self.state.call_stack.last_mut() {
                    Some(CallFrame::ByteCode { .. }) => {
                        self.state.stack[bottom] = self.state.stack[idx];
                        self.state.stack.resize(bottom + 1);
                    }
                    None => {
                        let ret = self.state.stack[idx];
                        self.state.result = Some(ret);
                        self.state.stack.clear();
                    }
                    _ => panic!("invalid state"),
                }
            }
            _ => panic!("not frame"),
        }
    }

    pub fn call(&mut self, function: usize, args: usize) -> Result<()> {
        match self.state.stack[function] {
            Value::Callable(Callable::Native(native)) => {
                let args = &self.state.stack.i[(function + 1)..(function + 1 + (args as usize))];
                let ret = native.call(args)?;
                self.native_return(ret);
            }
            Value::Callable(Callable::Closure(closure)) => {
                if args as u8 != closure.arity() {
                    panic!("invalid count");
                }

                let bottom = function;
                /*if method {
                    bottom -= 1;
                    self.state.stack.swap(bottom, func);
                }*/

                self.state.call_stack.push(CallFrame::ByteCode {
                    ip: 0,
                    //is_variable: false,
                    //method: method,
                    bottom,
                    base: function + 1 + args as usize,
                });

                //Ok(())
            }
            v => {
                panic!("not callable: {:?}", v);
            }
        }
        Ok(())
    }

    pub fn native_return(&mut self, value: Value<'gc>) {
        match self.state.call_stack.last_mut() {
            Some(CallFrame::ByteCode { .. }) => {
                self.state.stack.push(value);
            }
            None => {
                self.state.result = Some(value);
            }
            _ => {
                panic!("invalid state")
            }
        }
    }
}

pub struct BytecodeReader<'gc, 'a> {
    stack: &'a mut Stack<'gc>,
    ip: &'a mut usize,
    closure: Closure<'gc>,
    bottom: usize,
}

impl<'gc, 'a> BytecodeReader<'gc, 'a> {
    pub fn opcode(&mut self) -> Option<Opcode> {
        match self.read_byte() {
            Some(b) => Some(Opcode::try_from(b).unwrap()),
            None => None,
        }
    }

    #[inline]
    pub fn read_constant(&mut self) -> Option<Value<'gc>> {
        self.read_byte()
            .and_then(|byte| self.closure.chunk().constant(byte as usize))
    }

    #[inline]
    pub fn read_offset(&mut self) -> Option<u16> {
        let offset = byteorder::BE::read_u16(&self.closure.chunk().opcodes()[*self.ip..]);
        *self.ip += 2;
        Some(offset)
    }

    #[inline]
    pub fn read_local(&mut self) -> Option<u8> {
        self.read_byte().map(|i| self.bottom as u8 + i)
    }

    /*pub fn ip(&self) -> usize {
        *self.ip
    }*/

    #[inline]
    pub fn stack(&mut self) -> &mut Stack<'gc> {
        self.stack
    }

    #[inline]
    fn read_byte(&mut self) -> Option<u8> {
        let b = self.closure.chunk().get(*self.ip);
        *self.ip += 1;
        b
    }

    #[inline]
    pub fn to_offset(&mut self, offset: u16) {
        *self.ip += offset as usize;
    }
}
