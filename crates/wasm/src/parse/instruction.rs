use alloc::format;
use crate::module::Opcode;
use crate::module::WasmInstruction;
use crate::reader::ByteReader;
use crate::Result;

pub(crate) fn parse_instruction(reader: &ByteReader) -> Result<WasmInstruction> {
    let op = reader.read_u8()?;
    let op = Opcode::from_u8(op)?;
    Ok(match op {
        Opcode::LocalGet => {
            let idx = reader.read_leb128_u32()?;
            WasmInstruction::LocalGet(idx)
        }
        Opcode::LocalSet => {
            let idx = reader.read_leb128_u32()?;
            WasmInstruction::LocalSet(idx)
        }
        Opcode::I32Store => {
            let offset = reader.read_leb128_u32()?;
            let idx = reader.read_leb128_u32()?;
            WasmInstruction::I32Store { offset, idx }
        }
        Opcode::I32Store => {
            let offset = reader.read_leb128_u32()?;
            let idx = reader.read_leb128_u32()?;
            WasmInstruction::I64Store { offset, idx }
        }
        Opcode::I32Const => {
            let value = reader.read_leb128_i32()?;
            WasmInstruction::I32Const(value)
        }
        Opcode::I32Add => WasmInstruction::I32Add,
        Opcode::I64Add => WasmInstruction::I64Add,
        Opcode::End => WasmInstruction::End,
        Opcode::Call => {
            let idx = reader.read_leb128_u32()?;
            WasmInstruction::Call(idx)
        }
        _ => todo!("opcode not supported yet: {:?}", op)
    })
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use crate::module::{WasmFunctionBody, WasmInstruction};
    use crate::parse::WasmParser;

    #[test]
    fn parse_i32_store() {
        let wasm = hal_wat::WatParser::parse_str("(module (func (i32.store offset=4 (i32.const 4))))").unwrap();
        let result = WasmParser::parse(&wasm).unwrap();
        assert_eq!(result.codes.as_ref(), [
            WasmFunctionBody {
                locals: Box::default(),
                code: Box::new([
                    WasmInstruction::I32Const(4),
                    WasmInstruction::I32Store {
                        offset: 2,
                        idx: 4,
                    },
                    WasmInstruction::End,
                ]),
            }]);
    }
}