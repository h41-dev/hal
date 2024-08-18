use alloc::boxed::Box;

use crate::module::instruction::Instruction;
use crate::module::ValueTypes;

pub struct Signature {
    pub params: ValueTypes,
    pub results: ValueTypes,
}

enum Invoke {
    Function(Function)
}

pub struct Function {
    signature: Signature,
    locals: ValueTypes,
    instructions: Box<[Instruction]>,
}

