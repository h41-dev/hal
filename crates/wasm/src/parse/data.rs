use alloc::boxed::Box;
use alloc::vec;

use hal_core::reader::ByteReader;

use crate::module::WasmData;
use crate::Result;

pub(crate) fn parse_data_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmData]>> {
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;

    let mut result = vec![];

    for _ in 0..count {
        let memory_index = reader.read_leb128_u32()?;
        let offset = parse_expr(reader)?;
        let size = reader.read_leb128_u32()?;
        let data = reader.read_range(size as usize)?;

        result.push(WasmData {
            memory_index,
            offset,
            data,
        });
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}

fn parse_expr(reader: &ByteReader) -> Result<u32> {
    let _ = reader.read_leb128_u32()?;
    let offset = reader.read_leb128_u32()?;
    let _ = reader.read_leb128_u32()?;
    Ok(offset)
}
