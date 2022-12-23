use gc_arena::{Collect, GcCell, MutationContext};

use super::{bytecode::Bytecode, code::SectionId, ctx::BuildCtx, expression::Expression};
use crate::heap::{vec, Vec};

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct Section<'gc> {
    code: GcCell<'gc, Bytecode<'gc>>,
    pub index: usize,
}

impl<'gc> PartialEq for Section<'gc> {
    fn eq(&self, other: &Self) -> bool {
        GcCell::ptr_eq(self.code, other.code) && self.index == other.index
    }
}

impl<'gc> Eq for Section<'gc> {}

impl<'gc> Section<'gc> {
    pub fn push<E>(&self, mc: MutationContext<'gc, '_>, expr: E) -> &Self
    where
        E: Expression<'gc>,
    {
        self.code.write(mc).push(expr);
        self
    }

    pub fn bytecode_len(&self) -> usize {
        self.code.read().bytecode_len()
    }

    fn build(&self, ctx: &mut BuildCtx) {
        self.code.read().clone().build(ctx)
    }
}

#[derive(Debug, Collect)]
#[collect(no_drop)]
pub struct Sections<'gc> {
    sections: Vec<Section<'gc>>,
}

impl<'gc> Sections<'gc> {
    pub fn new() -> Sections<'gc> {
        Sections {
            sections: Vec::default(),
        }
    }

    pub fn create_section(&mut self, mc: MutationContext<'gc, '_>) -> Section<'gc> {
        let bytecode = Bytecode::default();
        let id = SectionId(self.sections.len());
        let section = Section {
            code: GcCell::allocate(mc, bytecode),
            index: self.sections.len(),
        };
        self.sections.push(section);
        section
    }

    pub fn create_map(&self) -> Vec<usize> {
        let mut sections = Vec::with_capacity(self.sections.len());
        let mut len = 0;
        for sec in self.sections.iter() {
            sections.push(len);
            len += sec.bytecode_len();
        }

        sections
    }

    pub fn build(&self, ctx: &mut BuildCtx) {
        for section in &self.sections {
            section.build(ctx);
        }
    }
}
