use crate::heap::Vec;

use super::{
    code::{ArgId, BinaryOperator, Code, ConstantId, UnaryOperator},
    scope::Local,
};

pub struct Top;

impl<'gc> Expression<'gc> for Top {
    fn into_code(self, _codes: &mut Vec<Code<'gc>>) {}
}

pub trait Expression<'gc> {
    fn into_code(self, codes: &mut Vec<Code<'gc>>);
}

impl<'gc, F> Expression<'gc> for F
where
    F: FnOnce(&mut Vec<Code<'gc>>),
{
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        (self)(codes)
    }
}

impl<'gc> Expression<'gc> for Code<'gc> {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.push(self)
    }
}

impl<'gc> Expression<'gc> for Vec<Code<'gc>> {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.extend(self)
    }
}

impl<'gc> Expression<'gc> for BinaryOperator {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.push(Code::Binary(self))
    }
}

impl<'gc> Expression<'gc> for UnaryOperator {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.push(Code::Unary(self))
    }
}

impl<'gc> Expression<'gc> for ConstantId {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.push(Code::Constant(self))
    }
}

impl<'gc> Expression<'gc> for Local<'gc> {
    fn into_code(self, codes: &mut Vec<Code<'gc>>) {
        codes.push(Code::GetLocal(self))
    }
}

// impl Expression for ArgId {
//     fn into_code(self, codes: &mut Vec<Code>) {
//         codes.push(Code::GetArg(self))
//     }
// }

pub mod expr {
    use crate::bytecode::builder2::{code::SectionId, section::Section};

    use super::*;

    pub fn binary<'gc, A, B>(a: A, b: B, op: BinaryOperator) -> impl Expression<'gc>
    where
        A: Expression<'gc>,
        B: Expression<'gc>,
    {
        move |codes: &mut Vec<Code<'gc>>| {
            a.into_code(codes);
            b.into_code(codes);
            op.into_code(codes);
        }
    }

    macro_rules! binary {
        ($($name: ident => $op: ident),*) => {
            $(
                pub fn $name<'gc, A, B>(a: A, b: B) -> impl Expression<'gc>
                where
                    A: Expression<'gc>,
                    B: Expression<'gc>,
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

    pub fn lte<'gc, A, B>(a: A, b: B) -> impl Expression<'gc>
    where
        A: Expression<'gc>,
        B: Expression<'gc>,
    {
        move |codes: &mut Vec<Code<'gc>>| {
            a.into_code(codes);
            b.into_code(codes);
            BinaryOperator::Gt.into_code(codes);
            UnaryOperator::Not.into_code(codes);
        }
    }

    pub fn gte<'gc, A, B>(a: A, b: B) -> impl Expression<'gc>
    where
        A: Expression<'gc>,
        B: Expression<'gc>,
    {
        move |codes: &mut Vec<Code<'gc>>| {
            a.into_code(codes);
            b.into_code(codes);
            BinaryOperator::Lt.into_code(codes);
            UnaryOperator::Not.into_code(codes);
        }
    }

    pub fn pop<'gc>() -> impl Expression<'gc> {
        |codes: &mut Vec<Code<'gc>>| {
            codes.push(Code::Pop);
        }
    }

    pub fn jump<'gc>(section: Section<'gc>) -> impl Expression<'gc> {
        move |codes: &mut Vec<Code<'gc>>| codes.push(Code::Jump(section))
    }

    pub fn jump_if_false<'gc, E>(expr: E, section: Section<'gc>) -> impl Expression<'gc>
    where
        E: Expression<'gc>,
    {
        move |codes: &mut Vec<Code<'gc>>| {
            expr.into_code(codes);
            codes.push(Code::JumpIfFalse(section));
        }
    }

    pub fn returns<'gc, E: Expression<'gc>>(returns: E) -> impl Expression<'gc> {
        move |codes: &mut Vec<Code<'gc>>| {
            returns.into_code(codes);
            codes.push(Code::Return);
        }
    }
}
