use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::RefCell;

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

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct ProcessState {
    functions: Box<[Rc<Function>]>,
    exports: Box<[Rc<Export>]>,
    memories: Box<[Rc<Memory>]>,
}

// FIXME own representation -- load from compiled module
impl ProcessState {
    pub fn new(module: Module) -> Result<Self, ProcessStateError> {
        Ok(Self {
            functions: module.functions
                .into_vec()
                .into_iter()
                .map(Rc::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),

            exports: module.exports
                .into_vec()
                .into_iter()
                .map(Rc::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),

            memories: module.memories
                .into_vec()
                .into_iter()
                .map(Rc::new)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    pub fn get_function(&self, idx: FunctionIndex) -> Result<Rc<Function>, Trap> {
        self.functions.get(idx as usize).ok_or(Trap::NotFoundLocalFunction(idx)).map(|rc| rc.clone())
    }

    pub fn get_export(&self, name: impl Into<String>) -> Result<Rc<Export>, Trap> {
        let name = name.into();
        self.exports.iter().find(|export| export.name().eq(&name))
            .map(|rc| rc.clone())
            .ok_or(Trap::NotFoundExportedFunction(name))
    }

    pub fn get_memory(&self, idx: MemoryIndex) -> Result<Rc<Memory>, Trap> {
        self.memories.get(idx as usize).ok_or(Trap::NotFoundMemory(idx)).map(|rc| rc.clone())
    }

}