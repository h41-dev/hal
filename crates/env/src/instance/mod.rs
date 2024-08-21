use alloc::rc::Rc;
use alloc::string::String;

use hal_core::module::{Memory, Value};
use hal_core::module::MemoryAddress;
use hal_process::{Process, Processor, Trap};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Instance{
    // pub(crate) processor: Rc<Processor>,
    pub(crate) process: Process,
}

impl Instance{
    pub fn invoke(&self, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Option<Value>, Trap> {
        // FIXME instead of invoking process directly there should be a mailbox
        // let process = &mut self.process;
        // self.environment.processor.invoke(process, name, args)
        todo!()
    }

    pub fn memory(&self, idx: MemoryAddress) -> Result<Rc<Memory>, Trap> {
        self.process.state.memory(idx)
    }
}