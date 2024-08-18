use alloc::boxed::Box;
use alloc::vec;
use crate::Result;
use crate::module::{WasmMemory, WasmResizableLimit};
use crate::reader::ByteReader;

pub(crate) fn parse_memory_section(size: u32, reader: &ByteReader) -> Result<Box<[WasmMemory]>>{
    let expected_reader_pos = reader.pos() + size as usize;
    let count = reader.read_leb128_u32()?;
    let mut result = vec![];

    for _ in 0..count {
        let limits = parse_limits(reader)?;
        result.push(WasmMemory { limits })
    }

    debug_assert_eq!(reader.pos(), expected_reader_pos);
    Ok(result.into())
}

fn parse_limits(reader: &ByteReader) -> Result<WasmResizableLimit> {
    let flags = reader.read_leb128_u32()?;
    let min = reader.read_leb128_u32()?;

    let max = if flags == 0 {
        None
    } else {
        let max = reader.read_leb128_u32()?;
        Some(max)
    };

    Ok(WasmResizableLimit { min, max })
}

#[cfg(test)]
mod tests {
    use crate::module::{WasmMemory, WasmResizableLimit};
    use crate::parse::WasmParser;

    #[test]
    fn parse_memory_no_max() {
        let wasm = hal_wat::WatParser::parse_str("(module (memory 1))").unwrap();
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.memories.as_ref(), [
            WasmMemory { limits: WasmResizableLimit { min: 1, max: None } }
        ])
    }

    #[test]
    fn parse_memory() {
        let wasm = hal_wat::WatParser::parse_str("(module (memory 1 2))").unwrap();
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.memories.as_ref(), [
            WasmMemory { limits: WasmResizableLimit { min: 1, max: Some(2) } }
        ])
    }
}
