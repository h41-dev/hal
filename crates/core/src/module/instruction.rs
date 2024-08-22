use crate::module::function::FunctionAddress;
use crate::module::memory::MemoryOffset;
use crate::module::MemoryFlags;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Instruction {
    AbsF32,
    AbsF64,

    AddF32,
    AddF64,
    AddI32,
    AddI64,

    AndI32,
    AndI64,

    Block(u32),
    BlockWithFuncType(u32, u32),
    BlockWithType(u32, u32),

    Br(u32),
    BrIf(u32),
    BrLabel(u32),
    BrTable(u32, u32),

    Call(u32),
    CallIndirect(u32, u32),

    CeilF32,
    CeilF64,

    CopysignF32,
    CopysignF64,

    ClzI32,
    ClzI64,

    ConstF32(f32),
    ConstF64(f64),
    ConstI32(i32),
    ConstI64(i64),

    CtzI32,
    CtzI64,

    DemoteF64F32,
    DivSI32,
    DivUI32,
    DivSI64,
    DivUI64,

    Drop128,
    Drop32,
    Drop64,
    DropRef,

    Else(u32),
    End,
    EndBlockFrame,

    EqF32,
    EqF64,
    EqI32,
    EqI64,

    EqzI32,
    EqzI64,

    ExtendI32SI64,
    ExtendI32UI64,
    Extend16SI32,
    Extend16SI64,
    Extend32SI64,
    Extend8SI32,
    Extend8SI64,

    ExtendI32SF32,
    ExtendI32SF64,
    ExtendI32UF32,
    ExtendI32UF64,

    ExtendI64SF32,
    ExtendI64SF64,
    ExtendI64UF32,
    ExtendI64UF64,

    FloorF32,
    FloorF64,

    GeF32,
    GeF64,
    GeSI32,
    GeSI64,
    GeUI32,
    GeUI64,

    GlobalGet(u32),
    GlobalSet128(u32),
    GlobalSet32(u32),
    GlobalSet64(u32),
    GlobalSetRef(u32),

    GtF32,
    GtF64,
    GtSI32,
    GtSI64,
    GtUI32,
    GtUI64,

    LeF32,
    LeF64,
    LeSI32,
    LeSI64,
    LeUI32,
    LeUI64,

    LocalGet128(u32),
    LocalGet32(u32),
    LocalGet64(u32),
    LocalGetRef(u32),

    LocalSet128(u32),
    LocalSet32(u32),
    LocalSet64(u32),
    LocalSetRef(u32),

    LocalTee128(u32),
    LocalTee32(u32),
    LocalTee64(u32),
    LocalTeeRef(u32),

    Loop(u32),
    LoopWithFuncType(u32, u32),
    LoopWithType(u32, u32),

    LtF32,
    LtF64,
    LtSI32,
    LtSI64,
    LtUI32,
    LtUI64,

    MemoryCopy(u32, u32),
    MemoryFill(u32),
    MemoryGrow(u32),
    MemoryInit(u32, u32),
    MemorySize(u32),

    MulF32,
    MulF64,
    MulI32,
    MulI64,

    Nop,

    NeF32,
    NeF64,
    NeI32,
    NeI64,

    NegF32,
    NegF64,

    OrI32,
    OrI64,

    PopcntI32,
    PopcntI64,

    PromoteF32F64,

    RefFunc(u32),
    RefIsNull,
    RefNull(u32),

    ReinterpretF32I32,
    ReinterpretF64I64,
    ReinterpretI32F32,
    ReinterpretI64F64,

    RemSI32,
    RemSI64,
    RemUI32,
    RemUI64,

    Return,

    ReturnCall(u32),
    ReturnCallIndirect(u32, u32),

    RotlI32,
    RotlI64,
    RotrI32,
    RotrI64,

    Select128,
    Select32,
    Select64,
    SelectRef,

    ShlI32,
    ShlI64,

    ShrSI32,
    ShrSI64,
    ShrUI32,
    ShrUI64,

    SqrtF32,
    SqrtF64,

    StoreF32 { flags: MemoryFlags, offset: MemoryOffset },
    StoreF64 { flags: MemoryFlags, offset: MemoryOffset },
    StoreI32 { flags: MemoryFlags, offset: MemoryOffset },
    StoreI64 { flags: MemoryFlags, offset: MemoryOffset },
    Store16I32 { flags: MemoryFlags, offset: MemoryOffset },
    Store16I64 { flags: MemoryFlags, offset: MemoryOffset },
    Store32I64 { flags: MemoryFlags, offset: MemoryOffset },
    Store8I32 { flags: MemoryFlags, offset: MemoryOffset },
    Store8I64 { flags: MemoryFlags, offset: MemoryOffset },

    SubF32,
    SubF64,
    SubI32,
    SubI64,

    TableCopy { from: u32, to: u32 },
    TableFill(u32),
    TableGet(u32),
    TableGrow(u32),
    TableInit(u32, u32),
    TableSet(u32),
    TableSize(u32),

    TruncF32SI32,
    TruncF32SI64,
    TruncF32UI32,
    TruncF32UI64,
    TruncF64SI32,
    TruncF64SI64,
    TruncF64UI32,
    TruncF64UI64,

    TruncSatF32SI32,
    TruncSatF32SI64,
    TruncSatF32UI32,
    TruncSatF32UI64,
    TruncSatF64SI32,
    TruncSatF64SI64,
    TruncSatF64UI32,
    TruncSatF64UI64,

    Unreachable,

    WrapI32I64,

    XorI32,
    XorI64
}