use alloc::string::String;
use core::fmt::{Display, Formatter, write};

use crate::module::{FunctionAddress, MemoryAddress, ValueType};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Trap {
    DivisionByZero(TrapDivisionByZero),

    NotFound(TrapNotFound),
    NotImplemented(TrapNotImplemented),

    Overflow(TrapOverflow),

    Type(TrapType),
    Underflow(TrapUnderflow),
}

impl Display for Trap {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Trap::DivisionByZero(t) => write!(f, "{}", t),
            Trap::NotFound(_) => todo!(),
            Trap::NotImplemented(t) => write!(f, "{}", t),
            Trap::Overflow(t) => write!(f, "{}", t),
            Trap::Type(t) => write!(f, "{}", t),
            Trap::Underflow(t) => write!(f, "{}", t),

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


#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapOverflow {
    Integer,
    Stack,
}

impl Display for TrapOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TrapOverflow::Integer => write!(f, "integer overflow"),
            TrapOverflow::Stack => write!(f, "stack overflow"),
        }
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapType {
    Mismatch(ValueType, ValueType)
}

impl Display for TrapType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TrapType::Mismatch(expected, got) => write!(f, "expected type {:?}, got {:?}", expected, got)
        }
    }
}


#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum TrapUnderflow {
    Stack
}

impl Display for TrapUnderflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            TrapUnderflow::Stack => write!(f, "stack underflow")
        }
    }
}