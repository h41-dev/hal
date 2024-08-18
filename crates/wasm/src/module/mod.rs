use alloc::boxed::Box;
use alloc::string::String;

pub(crate) use instruction::WasmInstruction;
pub(crate) use opcode::Opcode;

mod instruction;
mod opcode;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents a complete WebAssembly module, containing all standard sections.
pub struct WasmModule {
    /// The magic number identifying the file as a WebAssembly module (`"\0asm"`).
    pub magic: Box<[u8]>,

    /// The version of the WebAssembly module (usually `0x1` for current modules).
    pub version: u32,

    /// A boxed slice of custom sections, which can contain arbitrary data.
    pub customs: Box<[WasmCustom]>,

    /// A boxed slice of  function signatures.
    pub types: Box<[WasmFunc]>,

    /// A boxed slice of  imports (functions, tables, memories, globals).
    pub imports: Box<[WasmImport]>,

    /// A boxed slice of  function indices, each referring to a function signature in the types section.
    pub functions: Box<[u32]>,

    /// A boxed slice of  table types, specifying the types of elements in the table and its limits.
    pub tables: Box<[WasmTable]>,

    /// A boxed slice of  memory types, each defining the limits for the memory.
    pub memories: Box<[WasmMemory]>,

    /// A boxed slice of  exports, each with a name and description of what is being exported.
    pub exports: Box<[WasmExport]>,

    /// The index of the function to be called as the start function.
    pub start_function: Option<u32>,

    /// A boxed slice of  elements, each with a table index, offset, and initialization data.
    pub elements: Box<[WasmElement]>,

    /// A boxed slice of  function bodies, each containing local variable declarations and code.
    pub codes: Box<[WasmFunctionBody]>,

    /// A boxed slice of  data segments, each with a memory index, offset, and data.
    pub data: Box<[WasmData]>,

}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents a custom section in the Wasm module, containing arbitrary data.
pub struct WasmCustom {
    /// The name of the custom section.
    pub name: String,

    /// The raw data of the custom section.
    pub data: Box<[u8]>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Default, PartialEq)]
/// Represents a function signature, defining the parameter and return types.
pub struct WasmFunc {
    ///A boxed slice of parameter types.
    pub params: Box<[WasmValueType]>,

    ///A boxed slice of return types.
    pub returns: Box<[WasmValueType]>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents an import, specifying a module, name, and description of the imported item.
pub struct WasmImport {
    /// The module from which the item is imported.
    pub module: Box<[u8]>,

    /// The name of the item being imported.
    pub name: Box<[u8]>,

    /// A description of the imported item (function, table, memory, or global).
    pub desc: WasmImportDescriptor,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents the types of a table, specifying the types of elements and limits on the table size.
pub struct WasmTable {
    /// The types of elements in the table.
    pub element_type: u8,

    /// The limits on the table's size.
    pub limits: WasmResizableLimit,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents the types of a memory, specifying the limits on its size.
pub struct WasmMemory {
    /// The limits on the memory's size.
    pub limits: WasmResizableLimit,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents an export, specifying the name and description of what is being exported.
pub struct WasmExport {
    /// The name of the exported item.
    pub name: Box<[u8]>,

    /// A description of the exported item (function, table, memory, or global).
    pub desc: WasmExportDescriptor,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents an element in the element section, which is used to initialize tables.
pub struct WasmElement {
    /// The index of the table to initialize.
    pub table_index: u32,

    /// The offset in the table where the initialization begins.
    pub offset: Box<[Opcode]>,

    /// The list of function indices to place in the table.
    pub init: Box<[u32]>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents a function body in the code section, including local variable declarations and code.
pub struct WasmFunctionBody {
    ///A boxed slice of local variable declarations (count and types).
    pub locals: Box<[(u32, WasmValueType)]>,

    /// The instructions (opcodes) that make up the function body.
    pub code: Box<[WasmInstruction]>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents a data segment in the data section, which initializes a portion of memory.
pub struct WasmData {
    /// The index of the memory to initialize.
    pub memory_index: u32,

    /// The offset in the memory where the data begins.
    pub offset: u32,

    /// The raw data to be placed in the memory.
    pub data: Box<[u8]>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Describes the types of an import (function, table, memory, or global).
pub enum WasmImportDescriptor {
    /// Import a function with the given types index.
    Func(u32),

    /// Import a table with the given table types.
    Table(WasmTable),

    /// Import a memory with the given memory types.
    Memory(WasmMemory),
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Describes the types of an export (function, table, memory, or global).
pub enum WasmExportDescriptor {
    /// Export a function with the given index.
    Func(u32),

    /// Export a table with the given index.
    Table(u32),

    /// Export a memory with the given index.
    Memory(u32),

    /// Export a global with the given index.
    Global(u32),
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
/// Represents the limits on a resizable item (table or memory).
pub struct WasmResizableLimit {
    /// The minimum size of the table or memory.
    pub min: u32,

    /// The maximum size of the table or memory (optional).
    pub max: Option<u32>,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum WasmValueType {
    I32,
    I64,
}

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
}