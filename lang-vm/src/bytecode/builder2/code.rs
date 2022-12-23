use gc_arena::Collect;

use super::{ctx::BuildCtx, scope::Local, section::Section};
use crate::bytecode::Opcode;

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub struct LocalId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(require_static)]
pub struct ConstantId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(require_static)]
pub struct SectionId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(require_static)]
pub struct ArgId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(require_static)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
}

impl BinaryOperator {
    pub fn build(self, ctx: &mut BuildCtx) {
        match self {
            BinaryOperator::Add => ctx.push(Opcode::Add),
            BinaryOperator::Sub => ctx.push(Opcode::Sub),
            BinaryOperator::Div => ctx.push(Opcode::Div),
            BinaryOperator::Mul => ctx.push(Opcode::Mul),
            BinaryOperator::Gt => ctx.push(Opcode::Gt),
            BinaryOperator::Lt => ctx.push(Opcode::Lt),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(require_static)]
pub enum UnaryOperator {
    Not,
}

impl UnaryOperator {
    pub fn build(self, ctx: &mut BuildCtx) {
        match self {
            UnaryOperator::Not => ctx.push(Opcode::Not),
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub enum Code<'gc> {
    Binary(BinaryOperator),
    Unary(UnaryOperator),
    Constant(ConstantId),
    GetLocal(Local<'gc>),
    SetLocal(Local<'gc>),
    Call(u8),
    Jump(Section<'gc>),
    JumpIfFalse(Section<'gc>),
    GetArg(ArgId),
    Return,
    Pop,
}

impl<'gc> Code<'gc> {
    pub fn bytecode_len(&self) -> usize {
        use Code::*;
        match self {
            Binary(_) | Unary(_) | Return | Pop => 1,
            Constant(_) | GetLocal(_) | SetLocal(_) | GetArg(_) => 2,
            Call(_) => 1,
            Jump(_) | JumpIfFalse(_) => 3,
        }
    }

    pub fn build(self, ctx: &mut BuildCtx) {
        use Code::*;
        match self {
            Binary(op) => op.build(ctx),
            Unary(op) => op.build(ctx),
            Constant(c) => {
                ctx.push(Opcode::Constant);
                ctx.push(c.0 as u8);
            }
            GetLocal(c) => {
                ctx.push(Opcode::GetLocal);
                //let idx = ctx.local_map[c.0];
                ctx.push(c.index as u8); //(c.0 as u8);
            }
            SetLocal(c) => {
                //
                ctx.push(Opcode::SetLocal);
                let idx = 0u8; //ctx.local_map[c.0];
                ctx.push(idx);
            }
            GetArg(arg) => {
                ctx.push(Opcode::GetLocal);
                ctx.push((arg.0 + 1) as u8);
            }
            Call(arity) => {
                //
                let opcode = match arity {
                    0 => Opcode::Call0,
                    1 => Opcode::Call1,
                    2 => Opcode::Call2,
                    3 => Opcode::Call3,
                    _ => panic!("call"),
                };

                ctx.push(opcode);
            }
            Jump(section) => {
                let current = ctx.opcodes.len() + 3;
                let idx = ctx.section_map[section.index];
                let idx = (idx - current) as i16;

                ctx.push(Opcode::Jump);
                ctx.push(idx);
            }
            JumpIfFalse(section) => {
                let current = ctx.opcodes.len() + 3;
                let idx = ctx.section_map[section.index];
                let idx = (idx - current) as i16;

                ctx.push(Opcode::JumpIfFalse);
                ctx.push(idx);
            }
            Return => {
                ctx.push(Opcode::Return);
            }
            Pop => {
                ctx.push(Opcode::Pop);
            }
        }
    }
}
