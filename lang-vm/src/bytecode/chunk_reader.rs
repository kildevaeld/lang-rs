use super::{chunk, Chunk, Opcode};
use crate::Value;
use byteorder::ByteOrder;

pub struct ChunkReader<'gc, 'a> {
    ip: &'a mut usize,
    chunk: &'a Chunk<'gc>,
}

impl<'gc, 'a> ChunkReader<'gc, 'a> {
    pub fn new(ip: &'a mut usize, chunk: &'a Chunk<'gc>) -> ChunkReader<'gc, 'a> {
        ChunkReader { ip, chunk }
    }
}

impl<'gc, 'a> ChunkReader<'gc, 'a> {
    pub fn opcode(&mut self) -> Option<Opcode> {
        match self.read_byte() {
            Some(b) => Some(Opcode::try_from(b).unwrap()),
            None => None,
        }
    }

    #[inline]
    pub fn read_constant(&mut self) -> Option<Value<'gc>> {
        self.read_byte()
            .and_then(|byte| self.chunk.constant(byte as usize))
    }

    #[inline]
    pub fn read_offset(&mut self) -> Option<i16> {
        let offset = byteorder::BE::read_i16(&self.chunk.opcodes()[*self.ip..]);
        *self.ip += 2;
        Some(offset)
    }

    #[inline]
    pub fn read_local(&mut self) -> Option<u8> {
        self.read_byte()
    }

    /*pub fn ip(&self) -> usize {
        *self.ip
    }*/

    #[inline]
    fn read_byte(&mut self) -> Option<u8> {
        let b = self.chunk.get(*self.ip);
        *self.ip += 1;
        b
    }

    #[inline]
    pub fn to_offset(&mut self, offset: i16) {
        *self.ip += offset as usize;
    }
}
