#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

pub use env::Environment;
pub use error::{EnvironmentError, LoadError};
pub use instance::Instance;
pub use load::{LoadWasm, LoadWat};
pub use source::{wasm_source, wat_source};
pub use spawn::{SpawnWasm, SpawnWat};

mod env;
mod source;
mod error;
mod load;
mod spawn;
mod instance;
