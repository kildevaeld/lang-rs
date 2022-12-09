use super::Opcode;
use crate::{heap::Vec, Value};
use gc_arena::Collect;

#[derive(Debug, Collect, Clone)]
#[collect(no_drop)]
pub struct Chunk<'gc> {
    constants: Vec<Value<'gc>>,
    opcodes: Vec<u8>,
}

impl<'gc> Chunk<'gc> {
    pub fn new(opcodes: Vec<u8>, constants: Vec<Value<'gc>>) -> Chunk<'gc> {
        Chunk { constants, opcodes }
    }

    pub fn get(&self, idx: usize) -> Option<u8> {
        self.opcodes.get(idx).map(|m| *m)
    }

    pub fn constant(&self, idx: usize) -> Option<Value<'gc>> {
        self.constants.get(idx).map(|m| *m)
    }

    pub fn dissemble<W: ::core::fmt::Write>(&self, writer: &mut W) -> ::core::fmt::Result {
        let mut ip = 0;

        loop {
            if ip >= self.opcodes.len() {
                break;
            }

            let opcode = Opcode::try_from(self.opcodes[ip]).expect("opcode");

            ip = opcode.dissemble(&self.opcodes, &self.constants, ip, writer)?;

            writeln!(writer, "")?;
        }

        Ok(())
    }

    pub fn opcodes(&self) -> &[u8] {
        self.opcodes.as_slice()
    }
}

impl<'gc> core::fmt::Display for Chunk<'gc> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.dissemble(f)
    }
}
