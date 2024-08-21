use alloc::boxed::Box;
use alloc::rc::Rc;

use crate::module::{Export, Function, Memory};

pub type ModuleId = u16;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Module {
    pub id: ModuleId,
    pub functions: Box<[Rc<Function>]>,
    pub exports: Box<[Rc<Export>]>,
    pub memories: Box<[Rc<Memory>]>,
}

impl Module {
    pub fn new(
        id: ModuleId,
        exports: Box<[Rc<Export>]>,
        functions: Box<[Rc<Function>]>,
        memories: Box<[Rc<Memory>]>,
    ) -> Self {
        Self {
            id,
            functions,
            exports,
            memories,
        }
    }
}
