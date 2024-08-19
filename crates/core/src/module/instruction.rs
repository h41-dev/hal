use crate::module::memory::MemoryOffset;
use crate::module::function::FunctionAddress;
use crate::module::function::LocalAddress;
use crate::module::memory::MemoryAddress;
use crate::module::MemoryFlag;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Clone)]
pub enum Instruction {
    AddI32,
    AddI64,

    ConstI32(i32),
    ConstI64(i64),

    End,

    Invoke(FunctionAddress),

    LocalGet(LocalAddress),

    LocalSet(LocalAddress),

    StoreI32 { flag: MemoryFlag, offset: MemoryOffset },
    StoreI64 { flag: MemoryFlag, offset: MemoryOffset },
}


