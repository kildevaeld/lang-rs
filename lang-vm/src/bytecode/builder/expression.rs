use crate::heap::Vec;

use super::code::{ArgId, BinaryOperator, Code, ConstantId, LocalId, UnaryOperator};

pub struct Top;

impl Expression for Top {
    fn into_code(self, codes: &mut Vec<Code>) {}
}

pub trait Expression {
    fn into_code(self, codes: &mut Vec<Code>);
}

impl<F> Expression for F
where
    F: FnOnce(&mut Vec<Code>),
{
    fn into_code(self, codes: &mut Vec<Code>) {
        (self)(codes)
    }
}

impl Expression for Code {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(self)
    }
}

impl Expression for Vec<Code> {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.extend(self)
    }
}

impl Expression for BinaryOperator {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(Code::Binary(self))
    }
}

impl Expression for UnaryOperator {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(Code::Unary(self))
    }
}

impl Expression for ConstantId {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(Code::Constant(self))
    }
}

impl Expression for LocalId {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(Code::GetLocal(self))
    }
}

impl Expression for ArgId {
    fn into_code(self, codes: &mut Vec<Code>) {
        codes.push(Code::GetArg(self))
    }
}

pub mod expr {
    use crate::bytecode::builder::code::SectionId;

    use super::*;

    pub fn binary<A, B>(a: A, b: B, op: BinaryOperator) -> impl Expression
    where
        A: Expression,
        B: Expression,
    {
        move |codes: &mut Vec<Code>| {
            a.into_code(codes);
            b.into_code(codes);
            op.into_code(codes);
        }
    }

    macro_rules! binary {
        ($($name: ident => $op: ident),*) => {
            $(
                pub fn $name<A, B>(a: A, b: B) -> impl Expression
                where
                    A: Expression,
                    B: Expression,
                {
                    binary(a, b, BinaryOperator::$op)
                }
            )*
        };
    }

    binary!(
        add => Add,
        sub => Sub,
        mul => Mul,
        div => Div,
        lt => Lt,
        gt => Gt
    );

    pub fn lte<A, B>(a: A, b: B) -> impl Expression
    where
        A: Expression,
        B: Expression,
    {
        move |codes: &mut Vec<Code>| {
            a.into_code(codes);
            b.into_code(codes);
            BinaryOperator::Gt.into_code(codes);
            UnaryOperator::Not.into_code(codes);
        }
    }

    pub fn gte<A, B>(a: A, b: B) -> impl Expression
    where
        A: Expression,
        B: Expression,
    {
        move |codes: &mut Vec<Code>| {
            a.into_code(codes);
            b.into_code(codes);
            BinaryOperator::Lt.into_code(codes);
            UnaryOperator::Not.into_code(codes);
        }
    }

    pub fn pop() -> impl Expression {
        |codes: &mut Vec<Code>| {
            codes.push(Code::Pop);
        }
    }

    pub fn jump(section: SectionId) -> impl Expression {
        move |codes: &mut Vec<Code>| codes.push(Code::Jump(section))
    }

    pub fn jump_if_false<E>(expr: E, section: SectionId) -> impl Expression
    where
        E: Expression,
    {
        move |codes: &mut Vec<Code>| {
            expr.into_code(codes);
            codes.push(Code::JumpIfFalse(section));
        }
    }

    pub fn returns<E: Expression>(returns: E) -> impl Expression {
        move |codes: &mut Vec<Code>| {
            returns.into_code(codes);
            codes.push(Code::Return);
        }
    }
}
