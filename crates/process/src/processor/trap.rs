use alloc::string::String;
use core::fmt::{Display, Formatter};
use hal_core::module::{FunctionIndex, MemoryIndex};
use crate::process::state::ProcessStateError;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum Trap{
    NotFoundExportedFunction(String),
    NotFoundFunction(String),
    NotFoundMemory(MemoryIndex),
    NotFoundModule(String),
    NotFoundReturnValue,

    NotFoundLocalFunction(FunctionIndex),
}

impl Display for Trap{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    todo!()
       // match self {
       //     Trap::NotFoundExportedFunction(_) => {}
       //     Trap::NotFoundFunction(_) => {}
       //     Trap::NotFoundMemory(_) => {}
       //     Trap::NotFoundModule(_) => {}
       //     Trap::NotFoundReturnValue => {}
       //     Trap::NotFoundLocalFunction(_) => {}
       // }
    }
}