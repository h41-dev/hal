use alloc::boxed::Box;

use WasmParseError::InvalidSectionCode;

use crate::error::WasmParseError;
use crate::error::WasmParseError::{InvalidMagicNumber, UnsupportedVersion};
use crate::module::WasmModule;
use crate::parse::code::parse_code_section;
use crate::parse::custom::parse_custom_section;
use crate::parse::data::parse_data_section;
use crate::parse::export::parse_export_section;
use crate::parse::function::parse_functions_section;
use crate::parse::import::parse_import_section;
use crate::parse::memory::parse_memory_section;
use crate::parse::r#type::parse_types_section;
use crate::reader::ByteReader;
use crate::Result;

mod code;
mod custom;
mod data;
mod export;
mod function;
mod import;
mod instruction;
mod memory;
mod name;
mod r#type;
mod value;


pub enum SectionCode {
    Custom = 0x00,
    Type = 0x01,
    Import = 0x02,
    Function = 0x03,
    Memory = 0x05,
    Export = 0x07,
    Code = 0x0a,
    Data = 0x0b,
}

impl SectionCode {
    fn from_u8(value: u8) -> Result<SectionCode> {
        match value {
            0x00 => Ok(SectionCode::Custom),
            0x01 => Ok(SectionCode::Type),
            0x02 => Ok(SectionCode::Import),
            0x03 => Ok(SectionCode::Function),
            0x05 => Ok(SectionCode::Memory),
            0x07 => Ok(SectionCode::Export),
            0x0a => Ok(SectionCode::Code),
            0x0b => Ok(SectionCode::Data),
            _ => Err(InvalidSectionCode(value)),
        }
    }
}

/// The `WasmParser` struct is responsible for decoding a WebAssembly (WASM) binary module
/// from a byte stream. It utilizes a `ByteReader` to sequentially read and interpret
/// the bytes that represent the WASM module's structure, such as the magic header and version.
pub struct WasmParser {}

impl WasmParser {
    /// Decodes the WASM module from the byte stream.
    ///
    /// This function reads the necessary parts of a WASM module
    /// It proceeds by interpreting these values and advancing the cursor position accordingly.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a successfully decoded `Module` or a `ParseError`
    /// if any part of the decoding process fails (e.g., due to an unexpected end of file or
    /// invalid data).
    pub fn parse(input: &[u8]) -> Result<WasmModule> {
        let reader = ByteReader::new(input);
        let magic = Self::parse_magic(&reader)?;
        let version = Self::parse_version(&reader)?;

        let mut result = WasmModule {
            magic,
            version,
            customs: Box::default(),
            types: Box::default(),
            imports: Box::default(),
            functions: Box::default(),
            tables: Box::default(),
            memories: Box::default(),
            exports: Box::default(),
            start_function: None,
            elements: Box::default(),
            codes: Box::default(),
            data: Box::default(),
        };

        while !reader.eof() {
            let (code, size) = Self::parse_section_header(&reader)?;
            match code {
                SectionCode::Custom => {
                    result.customs = parse_custom_section(size, &reader)?
                }
                SectionCode::Type => {
                    result.types = parse_types_section(size, &reader)?
                }
                SectionCode::Import => {
                    result.imports = parse_import_section(size, &reader)?
                }
                SectionCode::Function => {
                    result.functions = parse_functions_section(size, &reader)?
                }
                SectionCode::Memory => {
                    result.memories = parse_memory_section(size, &reader)?
                }
                SectionCode::Export => {
                    result.exports = parse_export_section(size, &reader)?
                }
                SectionCode::Code => {
                    result.codes = parse_code_section(size, &reader)?
                }
                SectionCode::Data => {
                    result.data = parse_data_section(size, &reader)?
                }
            }
        }

        Ok(result)
    }

    fn parse_magic(reader: &ByteReader) -> Result<Box<[u8]>> {
        let result = reader.read_range(4)?;
        if result.as_ref() != [0x00, 0x61, 0x73, 0x6D] {
            Err(InvalidMagicNumber)
        } else {
            Ok(result)
        }
    }

    fn parse_version(reader: &ByteReader) -> Result<u32> {
        let result = reader.read_u32()?;
        if result != 1 {
            Err(UnsupportedVersion(result))
        } else {
            Ok(result)
        }
    }

    fn parse_section_header(reader: &ByteReader) -> Result<(SectionCode, u32)> {
        let code = SectionCode::from_u8(reader.read_u8()?)?;
        let size = reader.read_leb128_u32()?;
        Ok((code, size))
    }
}


#[cfg(test)]
mod tests {
    use alloc::boxed::Box;
    use alloc::format;

    use crate::error::WasmParseError::{InvalidMagicNumber, UnexpectedEndOfFile, UnsupportedVersion};
    use crate::module::{WasmData, WasmExport, WasmExportDescriptor, WasmFunc, WasmFunctionBody, WasmImport, WasmImportDescriptor, WasmMemory, WasmResizableLimit, WasmValueType};
    use crate::module::WasmInstruction;
    use crate::parse::WasmParser;
    use crate::reader::ByteReader;

    #[test]
    fn nothing_to_decode() {
        let err = WasmParser::parse([0u8, 0].as_ref()).err().unwrap();
        assert_eq!(err, UnexpectedEndOfFile)
    }

    #[test]
    fn invalid_magic_number() {
        let err = WasmParser::parse(&[0x00, 0x6D, 0x73, 0x61]).err().unwrap();
        assert_eq!(err, InvalidMagicNumber)
    }
    
    #[test]
    fn invalid_version() {
        let given_bytes = &2_i32.to_le_bytes();
        let reader = ByteReader::new(given_bytes.as_ref());
        let err = WasmParser::parse_version(&reader).err().unwrap();
        assert_eq!(err, UnsupportedVersion(2))
    }
    
    #[test]
    fn parse_empty_module() {
        let wasm = fixture("module_empty.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), []);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), []);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.start_function.as_ref(), None);
        assert_eq!(result.codes.as_ref(), []);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_empty_function() {
        let wasm = fixture("func_empty.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [WasmFunc::default()]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(),
                   [WasmFunctionBody {
                       locals: Box::default(),
                       code: Box::new([WasmInstruction::End]),
                   }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_func_with_params() {
        let wasm = fixture("func_params.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [WasmFunc { params: Box::new([WasmValueType::I32, WasmValueType::I64]), returns: Box::default() }]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), [WasmFunctionBody {
            locals: Box::default(),
            code: Box::new([WasmInstruction::End]),
        }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_func_with_locals() {
        let wasm = fixture("func_locals.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [WasmFunc::default()]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.start_function, None);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.codes.as_ref(), [WasmFunctionBody {
            locals: Box::new([(1, WasmValueType::I32), (2, WasmValueType::I64)]),
            code: Box::new([WasmInstruction::End]),
        }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_add() {
        let wasm = fixture("add.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [WasmFunc { params: Box::new([WasmValueType::I32, WasmValueType::I32]), returns: Box::new([WasmValueType::I32]) }]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), [
            WasmExport {
                name: Box::from("add".as_bytes()),
                desc: WasmExportDescriptor::Func(0),
            }
        ]);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), [WasmFunctionBody {
            locals: Box::default(),
            code: Box::new([
                WasmInstruction::LocalGet(0),
                WasmInstruction::LocalGet(1),
                WasmInstruction::I32Add,
                WasmInstruction::End,
            ]),
        }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_func_call() {
        let wasm = fixture("func_call.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [
            WasmFunc { params: Box::new([WasmValueType::I32]), returns: Box::new([WasmValueType::I32]) }
        ]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0, 0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), [
            WasmExport {
                name: Box::from("call_doubler".as_bytes()),
                desc: WasmExportDescriptor::Func(0),
            }
        ]);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), [
            WasmFunctionBody {
                locals: Box::default(),
                code: Box::new([
                    WasmInstruction::LocalGet(0),
                    WasmInstruction::Call(1),
                    WasmInstruction::End,
                ]),
            },
            WasmFunctionBody {
                locals: Box::default(),
                code: Box::new([
                    WasmInstruction::LocalGet(0),
                    WasmInstruction::LocalGet(0),
                    WasmInstruction::I32Add,
                    WasmInstruction::End,
                ]),
            }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_import() {
        let wasm = fixture("import.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [
            WasmFunc { params: Box::new([WasmValueType::I32]), returns: Box::new([WasmValueType::I32]) }
        ]);
        assert_eq!(result.imports.as_ref(), [
            WasmImport {
                module: Box::from("env".as_bytes()),
                name: Box::from("add".as_bytes()),
                desc: WasmImportDescriptor::Func(0),
            }
        ]);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), []);
        assert_eq!(result.exports.as_ref(), [
            WasmExport {
                name: Box::from("call_add".as_bytes()),
                desc: WasmExportDescriptor::Func(1),
            }
        ]);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), [
            WasmFunctionBody {
                locals: Box::default(),
                code: Box::new([
                    WasmInstruction::LocalGet(0),
                    WasmInstruction::Call(0),
                    WasmInstruction::End,
                ]),
            }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_store() {
        let wasm = fixture("store.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), [
            WasmFunc { params: Box::default(), returns: Box::default() }
        ]);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), [0]);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), [
            WasmMemory { limits: WasmResizableLimit { min: 1, max: None } }
        ]);
        assert_eq!(result.exports.as_ref(), [
            WasmExport {
                name: Box::from("i32_store".as_bytes()),
                desc: WasmExportDescriptor::Func(0),
            }
        ]);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), [
            WasmFunctionBody {
                locals: Box::default(),
                code: Box::new([
                    WasmInstruction::I32Const(0),
                    WasmInstruction::I32Const(42),
                    WasmInstruction::I32Store {
                        offset: 2,
                        idx: 0,
                    },
                    WasmInstruction::End,
                ]),
            }]);
        assert_eq!(result.data.as_ref(), []);
    }
    
    #[test]
    fn parse_data() {
        let wasm = fixture("data.wat");
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.magic.as_ref(), "\0asm".as_bytes());
        assert_eq!(result.version, 1);
        assert_eq!(result.customs.as_ref(), []);
        assert_eq!(result.types.as_ref(), []);
        assert_eq!(result.imports.as_ref(), []);
        assert_eq!(result.functions.as_ref(), []);
        assert_eq!(result.tables.as_ref(), []);
        assert_eq!(result.memories.as_ref(), [
            WasmMemory { limits: WasmResizableLimit { min: 1, max: None } }
        ]);
        assert_eq!(result.exports.as_ref(), []);
        assert_eq!(result.start_function, None);
        assert_eq!(result.codes.as_ref(), []);
        assert_eq!(result.data.as_ref(), [
            WasmData {
                memory_index: 0,
                offset: 0,
                data: Box::from("hello".as_bytes()),
            },
            WasmData {
                memory_index: 0,
                offset: 5,
                data: Box::from("world".as_bytes()),
            },
        ]);
    }

    fn fixture(file: &str) -> Box<[u8]> {
        hal_wat::WatParser::parse_file(format!("./fixture/{}", file)).unwrap()
    }
}
