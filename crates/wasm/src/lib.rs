#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;

pub use crate::error::WasmParseError;
pub use crate::parse::WasmParser;
pub use crate::module::*;

pub mod reader;

mod error;
mod module;
mod parse;

pub(crate) type Result<T, E = WasmParseError> = core::result::Result<T, E>;