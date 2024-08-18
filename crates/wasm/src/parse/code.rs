use alloc::boxed::Box;
use alloc::vec;
use crate::Result;
use crate::module::WasmFunctionBody;
use crate::parse::instruction::parse_instruction;
use crate::parse::value::parse_value_type;
use crate::reader::ByteReader;

pub(crate) fn parse_code_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmFunctionBody]>>{
    let mut result = vec![];
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;

    for _ in 0..count {
        let size = reader.read_leb128_u32()?;
        let body = parse_function_body(size, reader)?;
        result.push(body);
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}

fn parse_function_body(size: u32, reader: &ByteReader) -> Result<WasmFunctionBody>{
    let expected_reader_pos = reader.pos() + size as usize;

    let count = reader.read_leb128_u32()?;
    let mut locals = vec![];

    for _ in 0..count {
        let type_count = reader.read_leb128_u32()?;
        let value_type = parse_value_type(reader)?;
        locals.push((type_count, value_type));
    }

    let mut code = vec![];
    while reader.pos() < expected_reader_pos {
        let inst = parse_instruction(reader)?;
        code.push(inst);
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(WasmFunctionBody {
        locals: locals.into(),
        code: code.into(),
    })
}