#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(PartialEq)]
pub enum WasmParseError {
    InvalidMagicNumber,
    UnsupportedVersion(u32),
    UnexpectedEndOfFile,
    InvalidLEB128Encoding,
    InvalidSectionCode(u8),
    OutOfBounds,
    // InvalidUtf8String,
    InvalidValueType(u8),
    // InvalidElementType(u8),
    // InvalidFunctionType(u8),
    // InvalidTableType(u8),
    // InvalidMemoryType(u8),
    // InvalidGlobalType(u8),
    InvalidImportDescriptor(u8),
    InvalidExportDescriptor(u8),
    InvalidOpcode(u8),
    // InvalidIndex,
    // UnknownSection(u8),
    // UnsupportedFeature(&'static str),
}

impl core::fmt::Display for WasmParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WasmParseError::InvalidMagicNumber => write!(f, "Invalid magic number"),
            WasmParseError::UnsupportedVersion(version) => write!(f, "Unsupported version: {}", version),
            WasmParseError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            WasmParseError::OutOfBounds => write!(f, "Index out of bounds"),
            WasmParseError::InvalidLEB128Encoding => write!(f, "Invalid encoding"),
            WasmParseError::InvalidSectionCode(code) => write!(f, "Invalid section code {}", code),
            // DecodingError::InvalidSectionId(id) => write!(f, "Invalid section ID: {}", id),
            // DecodingError::InvalidUtf8String => write!(f, "Invalid UTF-8 string"),
            WasmParseError::InvalidValueType(value_type) => write!(f, "Invalid value types: {}", value_type),
            // DecodingError::InvalidElementType(element_type) => write!(f, "Invalid element types: {}", element_type),
            // DecodingError::InvalidFunctionType(function_type) => write!(f, "Invalid function types: {}", function_type),
            // DecodingError::InvalidTableType(table_type) => write!(f, "Invalid table types: {}", table_type),
            // DecodingError::InvalidMemoryType(memory_type) => write!(f, "Invalid memory types: {}", memory_type),
            // DecodingError::InvalidGlobalType(global_type) => write!(f, "Invalid global types: {}", global_type),
            WasmParseError::InvalidImportDescriptor(descriptor) => write!(f, "Invalid import descriptor: {}", descriptor),
            WasmParseError::InvalidExportDescriptor(descriptor) => write!(f, "Invalid export descriptor: {}", descriptor),
            WasmParseError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {}", opcode),
            // DecodingError::InvalidIndex => write!(f, "Invalid index"),
            // DecodingError::UnknownSection(section_id) => write!(f, "Unknown section ID: {}", section_id),
            // DecodingError::UnsupportedFeature(feature) => write!(f, "Unsupported feature: {}", feature),
            // DecodingError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}
