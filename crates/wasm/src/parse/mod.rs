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
    use crate::error::WasmParseError::{InvalidMagicNumber, UnexpectedEndOfFile, UnsupportedVersion};
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
}
