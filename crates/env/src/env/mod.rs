use crate::Handle;

pub mod source;
pub mod single_threaded;
mod error;

pub trait Environment: Send + Sync {}
