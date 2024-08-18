use hal_compile::Compiler;
use hal_process::Processor;
pub use load::{LoadWasm, LoadWat};
pub use spawn::{SpawnWasm, SpawnWat};

mod load;
mod spawn;

pub struct SingleThreadedEnvironment {
    pub(crate) compiler: Compiler,
    pub(crate) processor: Processor,
}

impl SingleThreadedEnvironment {}

impl Default for SingleThreadedEnvironment {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Processor::default(),
        }
    }
}
