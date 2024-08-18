use core::ops::ControlFlow;
use core::ops::ControlFlow::Continue;

use crate::process::Process;
use crate::Trap;

pub mod trap;
pub(crate) mod invoke;

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
}