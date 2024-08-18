use alloc::string::String;
use alloc::vec::Vec;

use hal_core::module::Value;
use hal_process::{Process, Processor, Trap};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Handle<'processor> {
    pub(crate) processor: &'processor Processor,
    pub(crate) process: Process,
}

impl<'runtime> Handle<'runtime> {

    pub fn invoke(&mut self, name: impl Into<String>, args: Vec<Value>) -> Result<Option<Value>, Trap> {
        // FIXME instead of invoking process directly there should be a mailbox
        let process = &mut self.process;
        self.processor.invoke(process, name, args)
    }

}