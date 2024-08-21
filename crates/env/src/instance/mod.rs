use alloc::boxed::Box;
use alloc::rc::{Rc, Weak};
use alloc::string::String;

use hal_core::module::{Memory, Value};
use hal_core::module::MemoryAddress;
use hal_process::{Process, Processor, Trap};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Instance {
    pub(crate) processor: Weak<Processor>,
    pub(crate) process: Process,
}

impl Instance {
    pub fn invoke(&mut self, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Box<[Value]>, Trap> {
        // FIXME instead of invoking process directly there should be a mailbox
        let process = &mut self.process;
        self.processor.upgrade().unwrap().invoke(process, name, args)
    }

    pub fn memory(&self, idx: MemoryAddress) -> Result<Rc<Memory>, Trap> {
        self.process.state.memory(idx)
    }
}