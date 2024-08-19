use alloc::boxed::Box;
pub use crate::module::export::*;
pub use crate::module::function::*;
pub use crate::module::instruction::*;
pub use crate::module::memory::*;
pub use crate::module::value::*;

mod value;
mod function;
mod instruction;
mod import;
mod export;
mod memory;

pub struct Module {
    pub exports: Box<[Export]>,
    pub functions: Box<[Function]>,
    pub memories: Box<[Memory]>,
}

impl Module {
    pub fn new(
        exports: Box<[Export]>,
        functions: Box<[Function]>,
        memories: Box<[Memory]>,
    ) -> Self {
        Self {
            exports,
            functions,
            memories,
        }
    }
}
