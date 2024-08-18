use hal_compile::Compiler;
use hal_process::Processor;

pub struct Single {
    pub(crate) compiler: Compiler,
    pub(crate) processor: Processor,
}

impl Single {}

impl Default for Single {
    fn default() -> Self {
        Self {
            compiler: Compiler::default(),
            processor: Processor::default(),
        }
    }
}
