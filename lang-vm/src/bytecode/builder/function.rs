use gc_arena::MutationContext;

use crate::{
    bytecode::Chunk,
    heap::{ToString, Vec},
    String, Value,
};

use super::{
    bytecode::Bytecode,
    code::{ArgId, ConstantId, LocalId, SectionId},
    ctx::BuildCtx,
    expression::Expression,
    section::Sections,
};

pub struct Local<'gc> {
    pub name: String<'gc>,
}

pub struct Function<'gc, 'a> {
    mc: MutationContext<'gc, 'a>,
    constants: Vec<Value<'gc>>,
    locals: Vec<Local<'gc>>,
    sections: Sections,
    args: usize,
}

impl<'gc, 'a> Function<'gc, 'a> {
    pub fn new(mc: MutationContext<'gc, 'a>) -> Function<'gc, 'a> {
        Function {
            mc,
            constants: Vec::default(),
            locals: Vec::default(),
            sections: Sections::new(),
            args: 0,
        }
    }

    pub fn this(&self) -> LocalId {
        LocalId(0)
    }

    pub fn define_arg(&mut self) -> ArgId {
        let id = self.args;
        self.args += 1;
        ArgId(id)
    }

    pub fn define_local(&mut self, name: impl ToString) -> LocalId {
        let id = self.locals.len();
        self.locals.push(Local {
            name: String::new(self.mc, &name.to_string()),
        });

        LocalId(id)
    }

    pub fn constant(&mut self, value: Value<'gc>) -> ConstantId {
        let id = self.constants.len();
        self.constants.push(value);
        ConstantId(id)
    }

    pub fn create_section(&mut self) -> SectionId {
        self.sections.create_section()
    }

    pub fn push<E>(&mut self, expr: E) -> &mut Self
    where
        E: Expression,
    {
        self.sections.push(expr);
        self
    }

    pub fn build(self) -> crate::bytecode::Function<'gc> {
        let section_map = self.sections.create_map();
        let local_map = self
            .locals
            .iter()
            .enumerate()
            .map(|(idx, _)| (idx + self.args) as u8)
            .collect::<Vec<_>>();

        let mut ctx = BuildCtx {
            section_map,
            local_map,
            opcodes: Vec::default(),
        };

        self.sections.build(&mut ctx);

        let chunk = Chunk::new(ctx.opcodes, self.constants);

        crate::bytecode::Function::new(chunk, self.args as u8)
    }
}

impl<'gc, 'a> core::ops::Index<SectionId> for Function<'gc, 'a> {
    type Output = Bytecode;
    fn index(&self, index: SectionId) -> &Self::Output {
        &self.sections[index]
    }
}

impl<'gc, 'a> core::ops::IndexMut<SectionId> for Function<'gc, 'a> {
    fn index_mut(&mut self, index: SectionId) -> &mut Self::Output {
        &mut self.sections[index]
    }
}
