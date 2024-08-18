pub use crate::module ::export::*;
pub use crate::module::index::*;
pub use crate::module::value::*;
pub use crate::module::instruction::*;


mod index;
mod value;
mod invoke;
mod instruction;
mod import;
mod export;
mod memory;

pub type Offset = u32;

pub struct Module {}