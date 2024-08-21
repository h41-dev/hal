use alloc::string::String;
use core::fmt::{Display, Formatter};
use hal_core::module::MemoryAddress;
use hal_core::module::FunctionAddress;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum Trap{
    NotFoundExportedFunction(String),
    NotFoundFunction(String),
    NotFoundMemory(MemoryAddress),
    NotFoundModule(String),
    NotFoundReturnValue,

    NotFoundLocalFunction(FunctionAddress),
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