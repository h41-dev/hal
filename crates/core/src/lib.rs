#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate core;
extern crate alloc;

pub use trap::Trap;
pub use error::Error;

pub mod module;
pub mod constant;
pub mod leb128;
pub mod reader;
mod trap;
mod error;
