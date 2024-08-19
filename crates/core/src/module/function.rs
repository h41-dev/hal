use alloc::boxed::Box;

use crate::module::{ ValueType, ValueTypes};
use crate::module::instruction::Instruction;

pub type LocalAddress = u32;
pub type FunctionAddress = u32;


#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct FunctionSignature {
    params: ValueTypes,
    results: ValueTypes,
}

impl FunctionSignature {
    pub fn new(params: ValueTypes, results: ValueTypes) -> Self {
        Self {
            params,
            results,
        }
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub enum Function {
    Local(FunctionLocal)
}

impl Function {
    pub fn local(signature: FunctionSignature, locals: ValueTypes, instructions: Box<[Instruction]>) -> Self {
        Function::Local(FunctionLocal {
            signature,
            locals,
            instructions,
        })
    }
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct FunctionLocal {
    signature: FunctionSignature,
    locals: ValueTypes,
    instructions: Box<[Instruction]>,
}

impl FunctionLocal {
    pub fn result_count(&self) -> usize {
        self.signature.results.len()
    }

    pub fn parameter_count(&self) -> usize {
        self.signature.params.len()
    }

    pub fn locals(&self) -> &[ValueType] { self.locals.as_ref() }

    pub fn instructions(&self) -> Box<[Instruction]> { Box::from(self.instructions.clone()) }
}

