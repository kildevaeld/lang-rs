use super::{code::Code, ctx::BuildCtx, expression::Expression};
use alloc::vec::Vec;
use gc_arena::Collect;

#[derive(Debug, Default, Clone, PartialEq, Eq, Collect)]
#[collect(no_drop)]
pub struct Bytecode<'gc> {
    codes: Vec<Code<'gc>>,
}

impl<'gc> Bytecode<'gc> {
    pub fn bytecode_len(&self) -> usize {
        self.codes
            .iter()
            .fold(0, |prev, code| prev + code.bytecode_len())
    }

    pub fn push<E>(&mut self, expr: E) -> &mut Self
    where
        E: Expression<'gc>,
    {
        expr.into_code(&mut self.codes);
        self
    }

    pub fn build(self, ctx: &mut BuildCtx) {
        for code in self.codes {
            code.build(ctx);
        }
    }
}

// impl Expression for Bytecode {
//     fn into_code(self, codes: &mut Vec<Code>) {
//         codes.extend(self.codes)
//     }
// }
