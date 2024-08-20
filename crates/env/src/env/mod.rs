use hal_compile::Compiler;
use hal_process::Processor;
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
