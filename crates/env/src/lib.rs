#![no_std]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

extern crate alloc;
extern crate core;

pub use env::Environment;
pub use env::single_threaded::{LoadWasm, LoadWat, SingleThreadedEnvironment, SpawnWasm, SpawnWat};
pub use env::source::*;
pub use state::State;

mod env;
mod state;
