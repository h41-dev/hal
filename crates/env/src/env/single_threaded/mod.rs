use alloc::string::ToString;
use hal_compile::Compiler;
use hal_process::Processor;
pub use load::{LoadWasm, LoadWat};
pub use spawn::{SpawnWasm, SpawnWat};
use crate::env::{Environment};

mod load;
mod spawn;

pub struct SingleThreadedEnvironment {
    pub(crate) compiler: Compiler,
    pub(crate) processor: Processor
}


impl Default for SingleThreadedEnvironment {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Processor::default(),
        }
    }
}

impl Environment for SingleThreadedEnvironment {
    fn name(&self) -> &'static str {
        "single_threaded"
    }
}