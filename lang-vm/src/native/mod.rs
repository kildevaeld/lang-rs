use gc_arena::Collect;

use crate::{error::Result, value::Value};

pub trait NativeFunc<'gc>: Collect {
    fn call(&self, args: &[Value<'gc>]) -> Result<Value<'gc>>;
}
