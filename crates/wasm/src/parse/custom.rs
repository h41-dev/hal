use alloc::boxed::Box;
use alloc::vec;

use crate::module::WasmCustom;
use crate::reader::ByteReader;
use crate::Result;

pub(crate) fn parse_custom_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmCustom]>> {
    // let expected_reader_pos = reader.pos() + size as usize;
    let _ = reader.pos() + size as usize;
    // let count = reader.read_leb128_u32()?;

    let result = vec![];

    //FIXME implement me
    let _ = reader.read_range(size as usize)?;


    // debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}