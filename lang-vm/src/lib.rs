#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

#[macro_use]
mod macros;

pub mod bytecode;
mod call_frame;
mod call_stack;
mod callback;
pub mod error;
mod native;
mod stack;
mod thread;
mod value;
mod vm;

mod heap;

pub use self::{native::NativeFunc, value::*, vm::Vm};

pub use gc_arena as gc;
