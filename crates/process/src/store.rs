use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;

use hal_core::{module, Trap, TrapNotFound};
use hal_core::module::{Export, Memory};
use hal_core::module::FunctionAddress;
use hal_core::module::MemoryAddress;
use hal_core::module::Module;
use module::Function;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum StoreError {
    NotFoundFunction(String),
    NotFoundMemory(MemoryAddress),
    NotFoundModule(String),
    NotFoundTypes,
}


impl core::fmt::Display for StoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StoreError::NotFoundFunction(name) => write!(f, "Function not found: {}", name),
            StoreError::NotFoundModule(name) => write!(f, "Module not found: {}", name),
            StoreError::NotFoundMemory(addr) => write!(f, "Memory not found: {}", addr),
            StoreError::NotFoundTypes => write!(f, "Types not found"),
        }
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Store {
    functions: Box<[Rc<Function>]>,
    exports: Box<[Rc<Export>]>,
    memories: Box<[Rc<Memory>]>,
}

// FIXME own representation -- load from compiled module
impl Store {
    pub fn new(module: &Module) -> Result<Self, StoreError> {
        Ok(Self {
            functions: module.functions.clone(),
            exports: module.exports.clone(),
            memories: module.memories.clone(),
        })
    }

    pub fn function(&self, addr: FunctionAddress) -> Result<Rc<Function>, Trap> {
        self.functions.get(addr as usize).ok_or(Trap::NotFound(TrapNotFound::FunctionLocal(addr))).map(|rc| rc.clone())
    }

    pub fn export(&self, name: impl Into<String>) -> Result<Rc<Export>, Trap> {
        let name = name.into();
        self.exports.iter().find(|export| export.name().eq(&name))
            .map(|rc| rc.clone())
            .ok_or(Trap::NotFound(TrapNotFound::ExportedFunction(name)))
    }

    pub fn memory(&self, addr: MemoryAddress) -> Result<Rc<Memory>, Trap> {
        self.memories.get(addr as usize).ok_or(Trap::NotFound(TrapNotFound::Memory(addr))).map(|rc| rc.clone())
    }
}