use super::{Callable, String};
use crate::{error::Result, Closure};
use gc_arena::{Collect, MutationContext};

#[derive(Debug, Clone, Copy, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    String(String<'gc>),
    Bool(bool),
    Callable(Callable<'gc>),
    Integer(i64),
    Nil,
}

impl<'gc> Value<'gc> {
    pub fn as_closure(&self) -> Option<Closure<'gc>> {
        match self {
            Value::Callable(call) => call.as_closure(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String<'gc>> {
        match self {
            Value::String(str) => Some(*str),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => Some(false),
        }
    }
}

impl<'gc> Value<'gc> {
    pub fn binary_add(
        &self,
        _mc: MutationContext<'gc, '_>,
        value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        match (self, value) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            _ => panic!("cannot add"),
        }
    }

    pub fn binary_sub(
        &self,
        _mc: MutationContext<'gc, '_>,
        value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        match (self, value) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            _ => panic!("cannot add"),
        }
    }

    pub fn binary_div(
        &self,
        _mc: MutationContext<'gc, '_>,
        _value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        todo!()
    }

    pub fn binary_mul(
        &self,
        _mc: MutationContext<'gc, '_>,
        _value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        todo!()
    }

    pub fn binary_lt(
        &self,
        _mc: MutationContext<'gc, '_>,
        _value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        todo!()
    }

    pub fn binary_gt(
        &self,
        _mc: MutationContext<'gc, '_>,
        value: Value<'gc>,
    ) -> Result<Value<'gc>> {
        match (self, value) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Bool(a > &b)),
            _ => panic!("cannot gt"),
        }
    }
}
