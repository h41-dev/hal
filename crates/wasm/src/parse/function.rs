use alloc::boxed::Box;
use alloc::vec;
use crate::Result;
use crate::reader::ByteReader;

pub(crate) fn parse_functions_section(size: u32, reader: &ByteReader) -> Result<Box<[u32]>> {
    let mut result = vec![];
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;

    for _ in 0..count {
        let addr = reader.read_leb128_u32()?;
        result.push(addr);
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}