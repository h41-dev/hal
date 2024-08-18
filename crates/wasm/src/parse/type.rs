use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use crate::Result;
use crate::module::WasmFunc;
use crate::parse::value::parse_value_types;
use crate::reader::ByteReader;

pub(crate) fn parse_types_section(size:u32, reader: &ByteReader) -> Result<Box<[WasmFunc]>> {
    let mut result: Vec<WasmFunc> = vec![];
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;

    for _ in 0..count {
        let _ = reader.read_u8();
        let mut func = WasmFunc::default();

        let param_count = reader.read_leb128_u32()?;
        func.params = parse_value_types(param_count, reader)?;

        let return_count = reader.read_leb128_u32()?;
        func.returns = parse_value_types(return_count, reader)?;

        result.push(func);
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}
