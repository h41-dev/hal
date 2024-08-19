use crate::module::{FunctionIndex, LocalIndex, MemoryIndex, Offset, Value, ValueType};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Clone)]
pub enum Instruction {
    AddI32,
    AddI64,

    ConstI32(i32),
    ConstI64(i64),

    End,

    Invoke(FunctionIndex),

    LocalGet(LocalIndex),

    LocalSet(LocalIndex),

    StoreI32 { offset: Offset, idx: MemoryIndex },
    StoreI64 { offset: Offset, idx: MemoryIndex },
}


