use core::marker::PhantomData;

use crate::{
    bytecode::{ChunkReader, Opcode},
    error::Result,
    heap::HashMap,
    stack::{Stack, StackContext},
};

use super::module::{ByteCodeState, LoadedState, Module};

pub fn execute<'gc>(
    module: &Module<'gc, ByteCodeState<'gc>>,
) -> Result<Module<'gc, LoadedState<'gc>>> {
    let mut ip = 0;
    let mut reader = ChunkReader::new(&mut ip, &module.state.chunk);

    let mut exports = HashMap::default();
    let mut stack = Stack::default();

    loop {
        let opcode = match reader.opcode() {
            None => {
                //
                break;
            }
            Some(opcode) => opcode,
        };

        match opcode {
            Opcode::Export => {
                let name = stack.get(-2).unwrap().as_string().unwrap();
                let value = stack.pop().unwrap();

                stack.set(-1, value);
                exports.insert(name, value);
            }
            _ => {
                panic!("invalid opcode: {}", opcode);
            }
        }
    }

    Ok(Module {
        state: LoadedState { exports },
        _p: PhantomData,
    })
}
