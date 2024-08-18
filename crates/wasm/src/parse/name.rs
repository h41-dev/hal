use alloc::boxed::Box;
use crate::Result;
use crate::reader::ByteReader;

pub(crate) fn parse_name<'a>(reader: &'a ByteReader) -> Result<Box<[u8]>> {
    let size = reader.read_leb128_u32()?;
    let name = reader.read_range(size as usize)?;
    Ok(name)
}