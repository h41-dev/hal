use alloc::boxed::Box;
use alloc::vec;
use crate::error::WasmParseError::InvalidExportDescriptor;
use crate::Result;
use crate::module::{WasmExport, WasmExportDescriptor};
use crate::parse::name::parse_name;
use crate::reader::ByteReader;

pub(crate) fn parse_export_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmExport]>> {
    let expected_reader_position = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;
    let mut result = vec![];

    for _ in 0..count {
        let name = parse_name(reader)?;
        let export_kind = reader.read_u8()?;
        let addr = reader.read_leb128_u32()?;
        let desc = match export_kind {
            0x00 => Ok(WasmExportDescriptor::Func(addr)),
            _ => Err(InvalidExportDescriptor(export_kind)),
        }?;
        result.push(WasmExport { name, desc });
    }

    debug_assert_eq!(reader.pos(), expected_reader_position);
    Ok(result.into())
}

