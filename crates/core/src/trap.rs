use alloc::string::String;

use crate::module::{FunctionAddress, MemoryAddress};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Trap {
    DivisionByZero,

    NotFound(TrapNotFound),

    OverflowInteger,
    OverflowStack,

    UnderflowStack,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapNotFound {
    ExportedFunction(String),
    Function(String),
    FunctionLocal(FunctionAddress),
    Memory(MemoryAddress),
    Module(String),
    ReturnValue,
}