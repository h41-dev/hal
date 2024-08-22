use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

use hal_core::{module, Trap};
use hal_core::module::{Export, Memory};
use hal_core::module::FunctionAddress;
use hal_core::module::MemoryAddress;
use hal_core::module::Module;
use module::Function;


#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum ProcessStateError {
    NotFoundFunction(String),
    NotFoundMemory(MemoryAddress),
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

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct ProcessState {
    functions: Box<[Rc<Function>]>,
    exports: Box<[Rc<Export>]>,
    memories: Box<[Rc<Memory>]>,
}

// FIXME own representation -- load from compiled module
impl ProcessState {
    pub fn new(module: &Module) -> Result<Self, ProcessStateError> {
        Ok(Self {
            functions: module.functions.clone(),
            exports: module.exports.clone(),
            memories: module.memories.clone(),
        })
    }

    pub fn function(&self, addr: FunctionAddress) -> Result<Rc<Function>, Trap> {
        self.functions.get(addr as usize).ok_or(Trap::NotFoundLocalFunction(addr)).map(|rc| rc.clone())
    }

    pub fn export(&self, name: impl Into<String>) -> Result<Rc<Export>, Trap> {
        let name = name.into();
        self.exports.iter().find(|export| export.name().eq(&name))
            .map(|rc| rc.clone())
            .ok_or(Trap::NotFoundExportedFunction(name))
    }

    pub fn memory(&self, addr: MemoryAddress) -> Result<Rc<Memory>, Trap> {
        self.memories.get(addr as usize).ok_or(Trap::NotFoundMemory(addr)).map(|rc| rc.clone())
    }
}