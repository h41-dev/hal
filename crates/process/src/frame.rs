use alloc::boxed::Box;

use hal_core::module::{Instruction, Value};

pub type InstructionPointer = isize;
pub type StackPointer = usize;
pub type Arity = usize;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Frame {
    pub(crate) ip: InstructionPointer,
    pub(crate) sp: StackPointer,
    pub(crate) instructions: Box<[Instruction]>,
    pub(crate) arity: Arity,
    pub(crate) locals: Box<[Value]>,
}

