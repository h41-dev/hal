use alloc::string::String;
use core::fmt::{Display, Formatter, write};

use crate::module::{FunctionAddress, MemoryAddress};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Trap {
    DivisionByZero(TrapDivisionByZero),

    NotFound(TrapNotFound),
    NotImplemented(TrapNotImplemented),

    OverflowInteger,
    OverflowStack,

    UnderflowStack,
}

impl Display for Trap {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Trap::DivisionByZero(t) => write!(f, "{}", t),
            Trap::NotFound(_) => todo!(),
            Trap::NotImplemented(t) => write!(f, "{}", t),
            Trap::OverflowInteger => todo!(),
            Trap::OverflowStack => todo!(),
            Trap::UnderflowStack => todo!()
        }
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapDivisionByZero {
    Integer
}

impl Display for TrapDivisionByZero {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TrapDivisionByZero::Integer => write!(f, "integer divide by zero")
        }
    }
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

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapNotImplemented {
    Instruction(crate::module::Instruction)
}

impl Display for TrapNotImplemented {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TrapNotImplemented::Instruction(i) => write!(f, "instruction not implemented {:?}", i)
        }
    }
}