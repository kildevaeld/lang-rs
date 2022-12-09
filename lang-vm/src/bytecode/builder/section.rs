use super::{bytecode::Bytecode, code::SectionId, ctx::BuildCtx, expression::Expression};
use crate::heap::{vec, BTreeMap, Vec};

#[derive(Debug)]
pub struct Sections {
    sections: Vec<Bytecode>,
}

impl Sections {
    pub fn new() -> Sections {
        Sections {
            sections: vec![Bytecode::default()],
        }
    }

    pub fn create_section(&mut self) -> SectionId {
        let bytecode = Bytecode::default();
        let id = SectionId(self.sections.len());
        self.sections.push(bytecode);
        id
    }

    pub fn push<E>(&mut self, expr: E) -> &mut Self
    where
        E: Expression,
    {
        self.sections[0].push(expr);
        self
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

    pub fn build(self, ctx: &mut BuildCtx) {
        for section in self.sections {
            section.build(ctx);
        }
    }
}

impl core::ops::Index<SectionId> for Sections {
    type Output = Bytecode;
    fn index(&self, index: SectionId) -> &Self::Output {
        &self.sections[index.0]
    }
}

impl core::ops::IndexMut<SectionId> for Sections {
    fn index_mut(&mut self, index: SectionId) -> &mut Self::Output {
        &mut self.sections[index.0]
    }
}
