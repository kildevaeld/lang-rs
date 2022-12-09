use super::opcodes::Opcode;
use crate::{
    error::Result,
    stack::{Stack, StackContext},
    Value,
};
use core::panic;

pub trait Reader<'gc> {
    fn opcode(&mut self) -> Option<Opcode>;

    fn read_constant(&mut self) -> Option<Value<'gc>>;
    fn read_offset(&mut self) -> Option<u16>;
    fn read_local(&mut self) -> Option<u8>;

    fn ip(&self) -> usize;
}

pub trait Context<'gc> {
    fn stack(&mut self) -> &mut Stack<'gc>;
    fn returns(&mut self, value: Option<Value<'gc>>);
    fn call(&mut self, function: usize, args: usize) -> Result<()>;
}

pub fn execute<'gc, R, C>(mut ctx: C, mut reader: R, mut instructions: usize) -> Result<usize>
where
    R: Reader<'gc>,
    C: Context<'gc>,
{
    loop {
        let opcode = match reader.opcode() {
            Some(opcode) => opcode,
            None => break,
        };

        match opcode {
            Opcode::Pop => {
                ctx.stack().pop();
            }
            Opcode::Constant => {
                let constant = match reader.read_constant() {
                    Some(constant) => constant,
                    None => panic!("constant not found"),
                };

                ctx.stack().push(constant);
            }
            Opcode::Call0 | Opcode::Call1 | Opcode::Call2 | Opcode::Call3 => {
                let count = (opcode as u8) - (Opcode::Call0 as u8);
                let idx = ctx.stack().top() - 1 - count as usize;
                ctx.call(idx, count as usize)?;
                break;
            }
            Opcode::GetLocal => {
                let offset = match reader.read_local() {
                    Some(offset) => offset,
                    None => panic!("offset not found"),
                };
                ctx.stack().dup(offset as usize);
            }
            Opcode::Return => {
                let value = ctx.stack().pop();
                ctx.returns(value);
                break;
            }
            _ => {
                panic!("")
            }
        }

        instructions -= 1;

        if instructions == 0 {
            break;
        }
    }

    Ok(instructions)
}
