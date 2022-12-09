use core::marker::PhantomData;

use byteorder::ByteOrder;

use crate::{bytecode::Opcode, heap::Vec};

pub struct BuildCtx {
    pub section_map: Vec<usize>,
    pub local_map: Vec<u8>,
    pub opcodes: Vec<u8>,
}

impl BuildCtx {
    pub fn push<B: Byte>(&mut self, byte: B) -> &mut Self {
        byte.into_byte(&mut self.opcodes);
        self
    }
}

pub trait Byte {
    fn into_byte(self, opcodes: &mut Vec<u8>);
}

impl Byte for Opcode {
    fn into_byte(self, opcodes: &mut Vec<u8>) {
        opcodes.push(self as u8)
    }
}

impl Byte for u8 {
    fn into_byte(self, opcodes: &mut Vec<u8>) {
        opcodes.push(self)
    }
}

impl Byte for u16 {
    fn into_byte(self, opcodes: &mut Vec<u8>) {
        let idx = opcodes.len();
        opcodes.resize(opcodes.len() + 2, 0);
        byteorder::BE::write_u16(&mut opcodes[idx..], self);
    }
}
