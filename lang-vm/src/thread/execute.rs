use std::println;

use super::bytecode_frame::BytecodeFrame;
use crate::{bytecode::Opcode, error::Result, stack::StackContext, Value};
use gc_arena::MutationContext;

pub fn execute<'gc>(
    mut frame: BytecodeFrame<'gc, '_>,
    mc: MutationContext<'gc, '_>,
    mut instructions: u32,
) -> Result<u32> {
    let mut reader = frame.reader();

    loop {
        let opcode = match reader.opcode() {
            Some(opcode) => opcode,
            None => {
                instructions = 0;
                break;
            }
        };

        /*println!(
            "IP {} {} {:?}",
            reader.ip() - 1,
            opcode,
            reader.stack().top()
        );*/

        match opcode {
            Opcode::Pop => {
                reader.stack().pop();
            }
            Opcode::Constant => {
                let constant = match reader.read_constant() {
                    Some(constant) => constant,
                    None => panic!("constant not found"),
                };

                reader.stack().push(constant);
            }
            Opcode::Call0 | Opcode::Call1 | Opcode::Call2 | Opcode::Call3 => {
                let count = (opcode as u8) - (Opcode::Call0 as u8);
                let idx = reader.stack().top() - 1 - count as usize;
                frame.call(idx, count as usize)?;
                break;
            }
            Opcode::Add => {
                reader.stack().add(mc)?;
            }
            Opcode::Sub => {
                reader.stack().sub(mc)?;
            }
            Opcode::Div => {
                reader.stack().div(mc)?;
            }
            Opcode::Mul => {
                reader.stack().mul(mc)?;
            }
            Opcode::Lt => {
                reader.stack().lt(mc)?;
            }
            Opcode::Gt => {
                reader.stack().gt(mc)?;
            }
            Opcode::Not => {
                let val = match reader.stack().get(-1) {
                    Some(value) => value.as_bool().expect("boolean"),
                    None => panic!("invalid "),
                };

                trace!("not: {:?}", val);

                reader.stack().set(-1, Value::Bool(!val));
            }
            Opcode::Jump => {
                let offset = match reader.read_offset() {
                    Some(offset) => offset,
                    None => panic!("invalid jump"),
                };
                trace!("jump {}", offset);
                reader.to_offset(offset);
            }
            Opcode::JumpIfFalse => {
                let value = reader.stack().get(-1).and_then(|ret| ret.as_bool());
                let offset = match reader.read_offset() {
                    Some(offset) => offset,
                    None => panic!("invalid offset"),
                };
                if let Some(value) = value {
                    trace!("jump_if_false: {} => {}", value, offset);
                    if !value {
                        reader.to_offset(offset);
                    }
                } else {
                    panic!("invalid bool value")
                }
            }
            Opcode::GetLocal => {
                let offset = match reader.read_local() {
                    Some(offset) => offset,
                    None => panic!("offset not found"),
                };
                reader.stack().dup(offset as usize)?;
            }
            Opcode::Return => {
                let top = reader.stack().top();
                trace!("return: {}", top);
                frame.returns(top - 1);
                break;
            }
            _ => {
                todo!("opcode: {}", opcode);
            }
        }

        instructions -= 1;

        if instructions == 0 {
            break;
        }
    }

    Ok(instructions)
}
