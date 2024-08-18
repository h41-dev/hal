use crate::module::{FunctionIndex, LocalIndex, MemoryIndex, Offset, Value, ValueType};

pub enum Instruction {
    AddI32,

    ConstI32(i32),

    End,

    // Invokes a local function
    InvokeLocal(FunctionIndex),

    LocalGet(LocalIndex),

    LocalSet(LocalIndex),

    StoreI32 { offset: Offset, idx: MemoryIndex },
}


