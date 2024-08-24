use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use hal_compile::Compiler;
use hal_core::module::{Module, ModuleId, Value};
use hal_core::Trap;
use hal_process::{Process, Processor, ProcessState};
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
    pub(crate) processor: Rc<Processor>,
    pub(crate) modules: Vec<Module>,
    pub(crate) instances: Vec<Instance>,
}


impl Default for Environment {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Rc::new(Processor::default()),
            modules: vec![],
            instances: vec![],
        }
    }
}

impl Environment {
    pub fn invoke(&mut self, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Box<[Value]>, Trap> {
        // FIXME handle nothing intantiated yet
        let len = self.instances.len();
        let instance = self.instances.get_mut(len - 1).unwrap();
        instance.invoke(name, args)
    }

    pub fn instantiate(&mut self, id: ModuleId) -> Result<&mut Instance, EnvironmentError> {
        let module = self.modules.get(id as usize).unwrap();

        let process_state = ProcessState::new(&module).unwrap();
        let instance = Instance {
            processor: Rc::downgrade(&self.processor),
            process: Process::new(process_state),
        };


        self.instances.push(instance);

        Ok(&mut self.instances[0])
    }
}