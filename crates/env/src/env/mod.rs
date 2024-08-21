use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use hal_compile::Compiler;
use hal_core::module::{Module, ModuleId, Value};
use hal_process::{Process, Processor, ProcessState, Trap};
pub use load::{LoadWasm, LoadWat};
pub use spawn::{SpawnWasm, SpawnWat};

use crate::env::error::EnvironmentError;
use crate::Instance;

pub mod source;
pub mod error;
mod load;
mod spawn;


#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Environment {
    pub(crate) compiler: Compiler,
    pub(crate) processor: Processor,
    pub(crate) modules: Vec<Module>,
    pub(crate) instances: Vec<Instance>,
}


impl Default for Environment {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Processor::default(),
            modules: vec![],
            instances: vec![],
        }
    }
}

impl Environment {
    pub fn invoke(&mut self, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Vec<Value>, Trap> {
        Ok(vec![])
    }

    pub fn instantiate(&mut self, id: ModuleId) -> Result<&Instance, EnvironmentError> {
        let module = self.modules.get(id as usize).unwrap();

        let process_state = ProcessState::new(&module).unwrap();
        let state = Instance {
            process: Process {
                state: process_state,
                stack: vec![],
                call_stack: vec![],
            },
        };

        self.instances.push(state);

        Ok(&self.instances.last().unwrap())
    }
}