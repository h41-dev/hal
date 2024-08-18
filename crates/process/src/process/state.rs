use alloc::boxed::Box;
use alloc::string::{String, ToString};

use hal_core::module;
use hal_core::module::{Export, FunctionIndex, Memory, MemoryIndex};
use hal_core::module::Module;
use module::Function;

use crate::Trap;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum ProcessStateError {
    NotFoundFunction(String),
    NotFoundMemory(MemoryIndex),
    NotFoundModule(String),
    NotFoundTypes,
}


impl core::fmt::Display for ProcessStateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProcessStateError::NotFoundFunction(name) => write!(f, "Function not found: {}", name),
            ProcessStateError::NotFoundModule(name) => write!(f, "Module not found: {}", name),
            ProcessStateError::NotFoundMemory(addr) => write!(f, "Memory not found: {}", addr),
            ProcessStateError::NotFoundTypes => write!(f, "Types not found"),
        }
    }
}

// FIXME
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct ProcessState {
    pub functions: Box<[Function]>,
    pub exports: Box<[Export]>,
    pub memories: Box<[Memory]>,
}

// FIXME own representation -- load from compiled module
impl ProcessState {
    pub fn new(module: Module) -> Result<Self, ProcessStateError> {
        Ok(Self {
            functions: module.functions,
            exports: module.exports,
            memories: module.memories,
        })
    }

    pub fn function_ref(&self, idx: FunctionIndex) -> Result<&Function, Trap> {
        self.functions.get(idx as usize).ok_or(Trap::NotFoundLocalFunction(idx))
    }

    pub fn export_ref(&self, name: impl Into<String>) -> Result<&Export, Trap> {
        let name= name.into();
        self.exports.iter().find(|export| export.name().eq(&name))
            .ok_or(Trap::NotFoundExportedFunction(name))
    }
}