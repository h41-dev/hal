#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

pub use env::{LoadWasm, LoadWat, SpawnWasm, SpawnWat};
pub use env::Environment;
pub use env::error::{EnvironmentError, LoadError};
pub use env::source::{wasm_source, wat_source};
pub use instance::Instance;

mod env;
mod instance;
