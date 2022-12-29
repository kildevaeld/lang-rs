use super::{
    code::{ConstantId, SectionId},
    ctx::BuildCtx,
    expression::Expression,
    section::{Section, Sections},
};
use crate::{
    bytecode::{Chunk, Opcode},
    heap::{HashMap, Vec},
    string_interner::StringInterner,
    Callable, Closure, String, Value,
};
use gc_arena::{Collect, Gc, GcCell, MutationContext};

#[derive(Debug, Collect, Clone, Copy, PartialEq, Eq)]
#[collect(no_drop)]
pub struct Local<'gc> {
    name: String<'gc>,
    scope: Scope<'gc>,
    pub index: usize,
}

#[derive(Debug, Collect)]
#[collect(no_drop)]
struct ScopeInner<'gc> {
    locals: HashMap<String<'gc>, Local<'gc>>,
    locals_order: Vec<String<'gc>>,
    parent: Option<GcCell<'gc, ScopeInner<'gc>>>,
    string_interner: StringInterner<'gc>,
}

impl<'gc> ScopeInner<'gc> {
    fn resolve_local(&self, name: &str) -> Option<Local<'gc>> {
        if let Some(local) = self.locals.get(name) {
            Some(*local)
        } else if let Some(parent) = self.parent {
            parent.read().resolve_local(name)
        } else {
            None
        }
    }
}

#[derive(Debug, Collect, Clone, Copy)]
#[collect(no_drop)]
pub struct Scope<'gc>(GcCell<'gc, ScopeInner<'gc>>);

impl<'gc> PartialEq for Scope<'gc> {
    fn eq(&self, other: &Self) -> bool {
        GcCell::ptr_eq(self.0, other.0)
    }
}

impl<'gc> Eq for Scope<'gc> {}

impl<'gc> Scope<'gc> {
    pub fn new(mc: MutationContext<'gc, '_>, interner: StringInterner<'gc>) -> Scope<'gc> {
        Scope(GcCell::allocate(
            mc,
            ScopeInner {
                locals: HashMap::default(),
                locals_order: Vec::default(),
                parent: None,
                string_interner: interner,
            },
        ))
    }

    pub fn create_child(&self, mc: MutationContext<'gc, '_>) -> Scope<'gc> {
        Scope(GcCell::allocate(
            mc,
            ScopeInner {
                locals: HashMap::default(),
                locals_order: Vec::default(),
                parent: Some(self.0),
                string_interner: self.0.read().string_interner,
            },
        ))
    }

    pub fn define_local(&self, mc: MutationContext<'gc, '_>, name: impl AsRef<str>) -> Local<'gc> {
        let name = self.0.read().string_interner.intern_or_get(mc, name);

        let mut state = self.0.write(mc);

        let local = Local {
            name: name,
            scope: *self,
            index: state.locals_order.len(),
        };

        state.locals.insert(name, local);
        state.locals_order.push(name);

        local
    }

    pub fn resolve_local(&self, name: impl AsRef<str>) -> Option<Local<'gc>> {
        self.0.read().resolve_local(name.as_ref())
    }
}

enum ModuleItem<'gc> {
    Func(Function<'gc>),
}

impl<'gc> ModuleItem<'gc> {
    pub fn name(&self) -> String<'gc> {
        match self {
            ModuleItem::Func(func) => func.0.read().name,
        }
    }
}

pub struct Module<'gc> {
    scope: Scope<'gc>,
    items: Vec<ModuleItem<'gc>>,
}

impl<'gc> Module<'gc> {
    pub fn new(mc: MutationContext<'gc, '_>, interner: StringInterner<'gc>) -> Module<'gc> {
        Module {
            scope: Scope::new(mc, interner),
            items: Vec::default(),
        }
    }

    pub fn create_function(
        &mut self,
        mc: MutationContext<'gc, '_>,
        name: impl AsRef<str>,
    ) -> Function<'gc> {
        let name = self.scope.0.read().string_interner.intern_or_get(mc, name);
        let scope = self.scope.create_child(mc);
        //let func = Func

        let func = Function(GcCell::allocate(
            mc,
            FunctionInner {
                name,
                scope,
                sections: Sections::new(),
                arity: 0,
                constants: Vec::default(),
            },
        ));

        self.items.push(ModuleItem::Func(func));

        self.scope.define_local(mc, name);

        func
    }

    pub fn resolve_local(&self, name: impl AsRef<str>) -> Option<Local<'gc>> {
        self.scope.resolve_local(name)
    }

    pub fn build(&self, mc: MutationContext<'gc, '_>) -> crate::bytecode::Function<'gc> {
        let mut constants = Vec::default();
        let mut opcodes: Vec<u8> = Vec::default();

        for item in self.items.iter() {
            match item {
                ModuleItem::Func(func) => {
                    let func = func.build();
                    constants.push(Value::Callable(Callable::Closure(Closure::new(
                        mc, func, 0,
                    ))));

                    let idx = constants.len() - 1;

                    opcodes.push(Opcode::Constant as u8);
                    opcodes.push(idx as u8);
                    opcodes.push(Opcode::Export as u8);
                }
            }
        }

        let chunk = Chunk::new(opcodes, constants);

        crate::bytecode::Function::new(chunk, 0)
    }
}

#[derive(Debug, Collect)]
#[collect(no_drop)]
struct FunctionInner<'gc> {
    name: String<'gc>,
    scope: Scope<'gc>,
    sections: Sections<'gc>,
    arity: u32,
    constants: Vec<Value<'gc>>,
}

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub struct Function<'gc>(GcCell<'gc, FunctionInner<'gc>>);

impl<'gc> Function<'gc> {
    pub fn define_local(&self, mc: MutationContext<'gc, '_>, name: impl AsRef<str>) -> Local<'gc> {
        self.0.write(mc).scope.define_local(mc, name)
    }

    pub fn constant(&self, mc: MutationContext<'gc, '_>, value: Value<'gc>) -> ConstantId {
        let id = self.0.read().constants.len();
        self.0.write(mc).constants.push(value);
        ConstantId(id)
    }

    pub fn create_section(&self, mc: MutationContext<'gc, '_>) -> Section<'gc> {
        self.0.write(mc).sections.create_section(mc)
    }

    pub fn build(&self) -> crate::bytecode::Function<'gc> {
        let section_map = self.0.read().sections.create_map();
        let mut ctx = BuildCtx {
            section_map,
            local_map: Vec::default(),
            opcodes: Vec::default(),
        };

        self.0.read().sections.build(&mut ctx);

        let chunk = Chunk::new(ctx.opcodes, self.0.read().constants.clone());

        crate::bytecode::Function::new(chunk, self.0.read().arity as u8)
    }
}
