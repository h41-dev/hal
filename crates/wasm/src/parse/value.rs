use alloc::boxed::Box;
use alloc::vec;

use hal_core::reader::ByteReader;

use crate::error::WasmParseError::InvalidValueType;
use crate::module::WasmValueType;
use crate::Result;

pub(crate) fn parse_value_type(reader: &ByteReader) -> Result<WasmValueType> {
    let value_type = reader.read_u8()?;
    Ok(value_type_from_u8(value_type)?)
}

pub(crate) fn parse_value_types(size: u32, reader: &ByteReader) -> Result<Box<[WasmValueType]>> {
    let mut result = vec![];
    for _ in 0..size {
        result.push(parse_value_type(reader)?);
    }
    Ok(result.into())
}

fn value_type_from_u8(value: u8) -> Result<WasmValueType> {
    match value {
        0x7F => Ok(WasmValueType::I32),
        0x7E => Ok(WasmValueType::I64),
        _ => Err(InvalidValueType(value)),
    }
}
