use alloc::string::String;

use hal_core::module::MemoryIndex;
use hal_core::module::Module;

#[derive(Debug)]
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
#[derive(Debug)]
pub struct ProcessState {
    // pub functions: Vec<hal_core::module::Function>,
    // pub exports: HashMap<String, Export>,
    // pub memories: Vec<hal_core::module::Memory>,
}

// FIXME own representation -- load from compiled module
impl ProcessState {
    pub fn new(module: Module) -> Result<Self, ProcessStateError> {
        Ok(Self {
            // functions: module.functions.into(),
            // exports: module.exports,
            // memories: module.memories.into(),
        })
    }
}