use alloc::boxed::Box;
use alloc::vec;
use crate::error::WasmParseError;
use crate::Result;
use crate::module::{WasmImport, WasmImportDescriptor};
use crate::parse::name::parse_name;
use crate::reader::ByteReader;

pub(crate) fn parse_import_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmImport]>> {
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;

    let mut result = vec![];

    for _ in 0..count {
        let module = parse_name(reader)?;
        let name = parse_name(reader)?;
        let import_kind = reader.read_u8()?;
        let desc = match import_kind {
            0x00 => {
                let addr = reader.read_leb128_u32()?;
                Ok(WasmImportDescriptor::Func(addr))
            }
            _ => Err(WasmParseError::InvalidImportDescriptor(import_kind)),
        }?;

        result.push(WasmImport {
            module,
            name,
            desc,
        });
    }
    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}