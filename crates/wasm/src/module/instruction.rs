#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum WasmInstruction {
    LocalGet(u32),
    LocalSet(u32),
    I32Store { offset: u32, idx: u32 },
    I64Store { offset: u32, idx: u32 },
    I32Const(i32),
    I64Const(i64),
    End,
    I32Add,
    I64Add,
    Call(u32),
}