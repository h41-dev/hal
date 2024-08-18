#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

pub use crate::process::Process;
pub use crate::process::state::ProcessState;
pub use crate::processor::Processor;
pub use crate::processor::trap::Trap;

mod process;
mod processor;