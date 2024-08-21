use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use hal_compile::Compiler;
use hal_core::module::Value;
use hal_process::{Processor, Trap};
pub use load::{LoadWasm, LoadWat};
pub use spawn::{SpawnWasm, SpawnWat};

pub mod source;
pub mod error;
mod load;
mod spawn;

pub struct Environment {
    pub(crate) compiler: Compiler,
    pub(crate) processor: Processor,
}


impl Default for Environment {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Processor::default(),
        }
    }
}

impl Environment {
    pub fn invoke(&mut self, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Vec<Value>, Trap> {
        Ok(vec![])
    }
}