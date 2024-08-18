use alloc::string::String;
use alloc::vec::Vec;
use core::ops::ControlFlow;
use core::ops::ControlFlow::Continue;

use hal_core::module::Value;

use crate::process::Process;
use crate::Trap;

pub mod trap;
pub(crate) mod invoke;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Processor {}

impl Default for Processor {
    fn default() -> Self {
        Self {}
    }
}

impl Processor {
    #[inline(always)]
    fn next(&self, process: &mut Process) -> ControlFlow<Option<Trap>> {
        Continue(())
    }

    pub fn invoke(&self, fiber: &mut Process, name: impl Into<String>, args: Vec<Value>) -> Result<Option<Value>, Trap> {
        Ok(None)
    }
}