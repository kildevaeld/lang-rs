use crate::value::Value;
use crate::{
    error::{Error, Result},
    heap::Vec,
};
use gc_arena::{Collect, MutationContext};

pub struct StackIdx(isize);

impl StackIdx {
    #[inline]
    fn index<'gc>(&self, stack: &[Value<'gc>]) -> Option<usize> {
        let idx = if self.0 < 0 {
            stack.len() as isize + self.0
        } else {
            self.0
        } as usize;

        if idx < stack.len() {
            Some(idx)
        } else {
            None
        }
    }

    #[inline]
    fn get<'gc>(&self, stack: &[Value<'gc>]) -> Option<Value<'gc>> {
        if let Some(idx) = self.index(stack) {
            Some(stack[idx])
        } else {
            None
        }
    }
}

impl From<isize> for StackIdx {
    fn from(idx: isize) -> Self {
        StackIdx(idx)
    }
}

impl From<usize> for StackIdx {
    fn from(idx: usize) -> Self {
        StackIdx(idx as isize)
    }
}

impl From<i32> for StackIdx {
    fn from(idx: i32) -> Self {
        StackIdx(idx as isize)
    }
}

pub trait StackContext<'gc> {
    fn pop(&mut self) -> Option<Value<'gc>>;

    fn push(&mut self, value: Value<'gc>);

    fn top(&self) -> usize;

    fn dup<I: Into<StackIdx>>(&mut self, idx: I) -> Result<()>;

    fn add(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;
    fn sub(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;
    fn div(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;
    fn mul(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;
    fn lt(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;
    fn gt(&mut self, mc: MutationContext<'gc, '_>) -> Result<()>;

    fn get<I: Into<StackIdx>>(&self, idx: I) -> Option<Value<'gc>>;
}

macro_rules! binary {
    ($this: ident, $mc: ident, $method: ident) => {{
        let left = $this
            .get(-2isize)
            .ok_or(Error::InvalidStackIndex { index: -2 })?;

        let right = $this
            .get(-1isize)
            .ok_or(Error::InvalidStackIndex { index: -1 })?;

        let result = left.$method($mc, right)?;

        let ret_idx = StackIdx(-2).index(&$this.i).unwrap();

        $this.i[ret_idx] = result;

        $this.i.pop();

        Ok(())
    }};
}

impl<'gc> StackContext<'gc> for Stack<'gc> {
    #[inline]
    fn pop(&mut self) -> Option<Value<'gc>> {
        self.i.pop()
    }

    #[inline]
    fn push(&mut self, value: Value<'gc>) {
        self.i.push(value)
    }

    #[inline]
    fn top(&self) -> usize {
        self.i.len()
    }

    #[inline]
    fn dup<I: Into<StackIdx>>(&mut self, idx: I) -> Result<()> {
        let idx = idx.into();

        let value = match idx.get(&self.i) {
            Some(ret) => ret,
            None => return Err(Error::InvalidStackIndex { index: idx.0 }),
        };
        self.i.push(value);
        Ok(())
    }

    fn get<I: Into<StackIdx>>(&self, idx: I) -> Option<Value<'gc>> {
        idx.into().get(&self.i)
    }

    #[inline]
    fn add(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_add)
    }

    #[inline]
    fn sub(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_sub)
    }

    #[inline]
    fn div(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_div)
    }

    #[inline]
    fn mul(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_mul)
    }

    #[inline]
    fn lt(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_lt)
    }

    #[inline]
    fn gt(&mut self, mc: MutationContext<'gc, '_>) -> Result<()> {
        binary!(self, mc, binary_gt)
    }
}

#[derive(Debug, Collect, Default)]
#[collect(no_drop)]
pub struct Stack<'gc> {
    pub(crate) i: Vec<Value<'gc>>,
}

impl<'gc> Stack<'gc> {
    pub fn resize(&mut self, size: usize) {
        self.i.resize(size, Value::Nil);
    }

    pub fn clear(&mut self) {
        self.i.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.i.is_empty()
    }

    pub fn set<I: Into<StackIdx>>(&mut self, idx: I, value: Value<'gc>) {
        let idx = idx.into().index(&self.i).expect("index");
        self.i[idx] = value;
    }
}

impl<'gc> core::ops::Index<usize> for Stack<'gc> {
    type Output = Value<'gc>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.i[index]
    }
}

impl<'gc> core::ops::IndexMut<usize> for Stack<'gc> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.i[index]
    }
}
