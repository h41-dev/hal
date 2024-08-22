use alloc::string::String;
use crate::module::{FunctionAddress, MemoryAddress};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Trap{
    DivisionByZero,
    IntegerOverflow,

    NotFoundExportedFunction(String),
    NotFoundFunction(String),
    NotFoundMemory(MemoryAddress),
    NotFoundModule(String),
    NotFoundReturnValue,

    NotFoundLocalFunction(FunctionAddress),
}