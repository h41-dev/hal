#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

use hal_core::Trap;

pub use crate::process::Process;
pub use crate::processor::Processor;
pub use crate::store::Store;

mod process;
mod processor;
mod stack;
mod store;
mod instruction;

type Result<T> = core::result::Result<T, Trap>;
