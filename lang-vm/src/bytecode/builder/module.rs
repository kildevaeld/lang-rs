use gc_arena::MutationContext;

use super::Function;
use crate::{heap::HashMap, String};

pub enum ModuleItem<'gc, 'a> {
    Function(Function<'gc, 'a>),
}

pub struct Module<'gc, 'a> {
    mc: MutationContext<'gc, 'a>,
    items: HashMap<String<'gc>, ModuleItem<'gc, 'a>>,
}
