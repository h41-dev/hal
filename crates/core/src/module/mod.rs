use alloc::boxed::Box;
use alloc::rc::Rc;

pub use crate::module::export::*;
pub use crate::module::function::*;
pub use crate::module::index::*;
pub use crate::module::instruction::*;
pub use crate::module::memory::Memory;
pub use crate::module::value::*;

mod index;
mod value;
mod function;
mod instruction;
mod import;
mod export;
mod memory;

pub type Offset = u32;

pub struct Module {
    pub exports: Box<[Export]>,
    pub functions: Box<[Rc<Function>]>,
    pub memories: Box<[Memory]>,
}

impl Module {
    pub fn new(
        exports: Box<[Export]>,
        functions: Box<[Rc<Function>]>,
        memories: Box<[Memory]>,
    ) -> Self {
        Self {
            exports,
            functions,
            memories,
        }
    }
}