pub use crate::module::export::*;
pub use crate::module::function::*;
pub use crate::module::instruction::*;
pub use crate::module::memory::*;
pub use crate::module::module::*;
pub use crate::module::value::*;

mod value;
mod function;
mod instruction;
mod import;
mod export;
mod memory;
mod module;
