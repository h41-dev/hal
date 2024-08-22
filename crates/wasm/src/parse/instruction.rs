use hal_core::reader::ByteReader;

use crate::module::Opcode;
use crate::module::WasmInstruction;
use crate::Result;

pub(crate) fn parse_instruction(reader: &ByteReader) -> Result<WasmInstruction> {
    let op = reader.read_u8()?;
    let op = Opcode::from_u8(op)?;
    match op {
        Opcode::AddI32 => Ok(WasmInstruction::AddI32),
        Opcode::AddI64 => Ok(WasmInstruction::AddI64),
        Opcode::AddF32 => Ok(WasmInstruction::AddF32),
        Opcode::AddF64 => Ok(WasmInstruction::AddF64),

        Opcode::SubI32 => Ok(WasmInstruction::SubI32),
        Opcode::SubI64 => Ok(WasmInstruction::SubI64),
        Opcode::SubF32 => Ok(WasmInstruction::SubF32),
        Opcode::SubF64 => Ok(WasmInstruction::SubF64),

        Opcode::MulI32 => Ok(WasmInstruction::MulI32),
        Opcode::MulI64 => Ok(WasmInstruction::MulI64),
        Opcode::MulF32 => Ok(WasmInstruction::MulF32),
        Opcode::MulF64 => Ok(WasmInstruction::MulF64),

        Opcode::DivSI32 => Ok(WasmInstruction::DivSI32),
        Opcode::DivUI32 => Ok(WasmInstruction::DivUI32),
        Opcode::DivSI64 => Ok(WasmInstruction::DivSI64),
        Opcode::DivUI64 => Ok(WasmInstruction::DivUI64),

        Opcode::ConstI32 => {
            let value = reader.read_leb128_i32()?;
            Ok(WasmInstruction::ConstI32(value))
        }
        Opcode::ConstI64 => {
            let value = reader.read_leb128_i64()?;
            Ok(WasmInstruction::ConstI64(value))
        }
        Opcode::ConstF32 => {
            let value = reader.read_f32()?;
            Ok(WasmInstruction::ConstF32(value))
        }
        Opcode::ConstF64 => {
            let value = reader.read_f64()?;
            Ok(WasmInstruction::ConstF64(value))
        }

        Opcode::AndI32 => Ok(WasmInstruction::AndI32),
        Opcode::AndI64 => Ok(WasmInstruction::AndI64),

        Opcode::OrI32 => Ok(WasmInstruction::OrI32),
        Opcode::OrI64 => Ok(WasmInstruction::OrI64),

        // Opcode::XorI32 => Ok(WasmInstruction::XorI32),
        // Opcode::XorI64 => Ok(WasmInstruction::XorI64),

        Opcode::ShlI32 => Ok(WasmInstruction::ShlI32),
        Opcode::ShlI64 => Ok(WasmInstruction::ShlI64),

        Opcode::ShrSI32 => Ok(WasmInstruction::ShrSI32),
        Opcode::ShrSI64 => Ok(WasmInstruction::ShrSI64),
        Opcode::ShrUI32 => Ok(WasmInstruction::ShrUI32),
        Opcode::ShrUI64 => Ok(WasmInstruction::ShrUI64),

        Opcode::RotlI32 => Ok(WasmInstruction::RotlI32),
        Opcode::RotlI64 => Ok(WasmInstruction::RotlI64),
        Opcode::RotrI32 => Ok(WasmInstruction::RotrI32),
        Opcode::RotrI64 => Ok(WasmInstruction::RotrI64),

        Opcode::EqI32 => Ok(WasmInstruction::EqI32),
        Opcode::EqI64 => Ok(WasmInstruction::EqI64),
        Opcode::EqF32 => Ok(WasmInstruction::EqF32),
        Opcode::EqF64 => Ok(WasmInstruction::EqF64),

        Opcode::NeI32 => Ok(WasmInstruction::NeI32),
        Opcode::NeI64 => Ok(WasmInstruction::NeI64),
        Opcode::NeF32 => Ok(WasmInstruction::NeF32),
        Opcode::NeF64 => Ok(WasmInstruction::NeF64),

        Opcode::EqzI32 => Ok(WasmInstruction::EqzI32),
        Opcode::EqzI64 => Ok(WasmInstruction::EqzI64),

        Opcode::ClzI32 => Ok(WasmInstruction::ClzI32),
        Opcode::ClzI64 => Ok(WasmInstruction::ClzI64),

        Opcode::CtzI32 => Ok(WasmInstruction::CtzI32),
        Opcode::CtzI64 => Ok(WasmInstruction::CtzI64),

        Opcode::PopcntI32 => Ok(WasmInstruction::PopcntI32),
        Opcode::PopcntI64 => Ok(WasmInstruction::PopcntI64),

        Opcode::NegF32 => Ok(WasmInstruction::NegF32),
        Opcode::NegF64 => Ok(WasmInstruction::NegF64),

        Opcode::AbsF32 => Ok(WasmInstruction::AbsF32),
        Opcode::AbsF64 => Ok(WasmInstruction::AbsF64),

        Opcode::SqrtF32 => Ok(WasmInstruction::SqrtF32),
        Opcode::SqrtF64 => Ok(WasmInstruction::SqrtF64),

        Opcode::CeilF32 => Ok(WasmInstruction::CeilF32),
        Opcode::CeilF64 => Ok(WasmInstruction::CeilF64),

        Opcode::FloorF32 => Ok(WasmInstruction::FloorF32),
        Opcode::FloorF64 => Ok(WasmInstruction::FloorF64),

        Opcode::CopysignF32 => Ok(WasmInstruction::CopysignF32),
        Opcode::CopysignF64 => Ok(WasmInstruction::CopysignF64),

        // Opcode::MinF32 => Ok(WasmInstruction::MinF32),
        // Opcode::MinF64 => Ok(WasmInstruction::MinF64),
        //
        // Opcode::MaxF32 => Ok(WasmInstruction::MaxF32),
        // Opcode::MaxF64 => Ok(WasmInstruction::MaxF64),

        Opcode::Call => {
            let addr = reader.read_leb128_u32()?;
            Ok(WasmInstruction::Call(addr))
        }
        Opcode::CallIndirect => {
            let type_index = reader.read_leb128_u32()?;
            let table_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::CallIndirect(type_index, table_index))
        }

        Opcode::Br => {
            let label_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::Br(label_index))
        }
        Opcode::BrIf => {
            let label_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::BrIf(label_index))
        }
        Opcode::BrTable => {
            let table_index = reader.read_leb128_u32()?;
            let default_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::BrTable(table_index, default_index))
        }

        Opcode::Block => {
            let block_type = reader.read_leb128_u32()?;
            Ok(WasmInstruction::Block(block_type))
        }
        Opcode::Loop => {
            let block_type = reader.read_leb128_u32()?;
            Ok(WasmInstruction::Loop(block_type))
        }
        // Opcode::If => {
        //     let block_type = reader.read_leb128_u32()?;
        //     Ok(WasmInstruction::If(block_type))
        // }
        Opcode::Else => {
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::Else(offset))
        }

        Opcode::End => Ok(WasmInstruction::End),
        Opcode::Unreachable => Ok(WasmInstruction::Unreachable),

        Opcode::Drop => Ok(WasmInstruction::Drop),

        Opcode::GlobalGet => {
            let global_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::GlobalGet(global_index))
        }
        Opcode::GlobalSet => {
            let global_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::GlobalSet32(global_index))
        }

        Opcode::LocalGet => {
            let local_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::LocalGet32(local_index))
        }
        Opcode::LocalSet => {
            let local_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::LocalSet32(local_index))
        }
        Opcode::LocalTee => {
            let local_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::LocalTee32(local_index))
        }

        Opcode::LoadI32 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreI32 { flags, offset })
        }
        Opcode::LoadI64 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreI64 { flags, offset })
        }
        Opcode::LoadF32 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreF32 { flags, offset })
        }
        Opcode::LoadF64 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreF64 { flags, offset })
        }

        Opcode::StoreI32 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreI32 { flags, offset })
        }
        Opcode::StoreI64 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreI64 { flags, offset })
        }
        Opcode::StoreF32 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreF32 { flags, offset })
        }
        Opcode::StoreF64 => {
            let flags = reader.read_leb128_u32()?;
            let offset = reader.read_leb128_u32()?;
            Ok(WasmInstruction::StoreF64 { flags, offset })
        }

        Opcode::MemorySize => {
            let memory_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::MemorySize(memory_index))
        }
        Opcode::MemoryGrow => {
            let memory_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::MemoryGrow(memory_index))
        }
        Opcode::MemoryInit => {
            let segment_index = reader.read_leb128_u32()?;
            let memory_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::MemoryInit(segment_index, memory_index))
        }
        Opcode::MemoryCopy => {
            let source_index = reader.read_leb128_u32()?;
            let destination_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::MemoryCopy(source_index, destination_index))
        }
        Opcode::MemoryFill => {
            let memory_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::MemoryFill(memory_index))
        }

        // Opcode::TableGet => {
        //     let table_index = reader.read_leb128_u32()?;
        //     Ok(WasmInstruction::TableGet(table_index))
        // }
        // Opcode::TableSet => {
        //     let table_index = reader.read_leb128_u32()?;
        //     Ok(WasmInstruction::TableSet(table_index))
        // }
        Opcode::TableGrow => {
            let table_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::TableGrow(table_index))
        }
        Opcode::TableSize => {
            let table_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::TableSize(table_index))
        }
        Opcode::TableCopy => {
            let from_index = reader.read_leb128_u32()?;
            let to_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::TableCopy { from: from_index, to: to_index })
        }
        Opcode::TableInit => {
            let element_index = reader.read_leb128_u32()?;
            let table_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::TableInit(element_index, table_index))
        }
        Opcode::TableFill => {
            let table_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::TableFill(table_index))
        }

        Opcode::RefFunc => {
            let function_index = reader.read_leb128_u32()?;
            Ok(WasmInstruction::RefFunc(function_index))
        }
        Opcode::RefIsNull => Ok(WasmInstruction::RefIsNull),
        Opcode::RefNull => {
            let ref_type = reader.read_leb128_u32()?;
            Ok(WasmInstruction::RefNull(ref_type))
        }

        // Opcode::TruncF32SI32 => Ok(WasmInstruction::TruncF32SI32),
        // Opcode::TruncF32SI64 => Ok(WasmInstruction::TruncF32SI64),
        // Opcode::TruncF32UI32 => Ok(WasmInstruction::TruncF32UI32),
        // Opcode::TruncF32UI64 => Ok(WasmInstruction::TruncF32UI64),
        // Opcode::TruncF64SI32 => Ok(WasmInstruction::TruncF64SI32),
        // Opcode::TruncF64SI64 => Ok(WasmInstruction::TruncF64SI64),
        // Opcode::TruncF64UI32 => Ok(WasmInstruction::TruncF64UI32),
        // Opcode::TruncF64UI64 => Ok(WasmInstruction::TruncF64UI64),

        Opcode::PromoteF32F64 => Ok(WasmInstruction::PromoteF32F64),
        // Opcode::DemoteF64F32 => Ok(WasmInstruction::DemoteF64F32),

        Opcode::ReinterpretF32I32 => Ok(WasmInstruction::ReinterpretF32I32),
        Opcode::ReinterpretF64I64 => Ok(WasmInstruction::ReinterpretF64I64),
        Opcode::ReinterpretI32F32 => Ok(WasmInstruction::ReinterpretI32F32),
        Opcode::ReinterpretI64F64 => Ok(WasmInstruction::ReinterpretI64F64),

        Opcode::WrapI32I64 => Ok(WasmInstruction::WrapI32I64),

        // Opcode::ExtendI32SI64 => Ok(WasmInstruction::ExtendI32SI64),
        // Opcode::ExtendI32UI64 => Ok(WasmInstruction::ExtendI32UI64),
        Opcode::Extend16SI32 => Ok(WasmInstruction::Extend16SI32),
        Opcode::Extend16SI64 => Ok(WasmInstruction::Extend16SI64),
        Opcode::Extend32SI64 => Ok(WasmInstruction::Extend32SI64),
        Opcode::Extend8SI32 => Ok(WasmInstruction::Extend8SI32),
        Opcode::Extend8SI64 => Ok(WasmInstruction::Extend8SI64),

        // _ => Err(WasmParseError::UnsupportedOpcode(op))
        Opcode::Nop => todo!(),
        Opcode::If => todo!(),
        Opcode::Try => todo!(),
        Opcode::Catch => todo!(),
        Opcode::Throw => todo!(),
        Opcode::Rethrow => todo!(),
        Opcode::Return => todo!(),
        Opcode::Select => todo!(),
        Opcode::SelectT => todo!(),
        Opcode::Load8SI32 => todo!(),
        Opcode::Load8UI32 => todo!(),
        Opcode::Load16SI32 => todo!(),
        Opcode::Load16UI32 => todo!(),
        Opcode::Load8SI64 => todo!(),
        Opcode::Load8UI64 => todo!(),
        Opcode::Load16SI64 => todo!(),
        Opcode::Load16UI64 => todo!(),
        Opcode::Load32SI64 => todo!(),
        Opcode::Load32UI64 => todo!(),
        Opcode::Store8I32 => todo!(),
        Opcode::Store16I32 => todo!(),
        Opcode::Store8I64 => todo!(),
        Opcode::Store16I64 => todo!(),
        Opcode::Store32I64 => todo!(),
        Opcode::LtSI32 => Ok(WasmInstruction::LtSI32),
        Opcode::LtUI32 => Ok(WasmInstruction::LtUI32),
        Opcode::GtSI32 => Ok(WasmInstruction::GtSI32),
        Opcode::GtUI32 => Ok(WasmInstruction::GtUI32),
        Opcode::LeSI32 => Ok(WasmInstruction::LeSI32),
        Opcode::LeUI32 => Ok(WasmInstruction::LeUI32),
        Opcode::GeSI32 => Ok(WasmInstruction::GeSI32),
        Opcode::GeUI32 => Ok(WasmInstruction::GeUI32),
        Opcode::LtSI64 => Ok(WasmInstruction::LtSI64),
        Opcode::LtUI64 => Ok(WasmInstruction::LtUI64),
        Opcode::GtSI64 => Ok(WasmInstruction::GtSI64),
        Opcode::GtUI64 => Ok(WasmInstruction::GtUI64),
        Opcode::LeSI64 => Ok(WasmInstruction::LeSI64),
        Opcode::LeUI64 => Ok(WasmInstruction::LeUI64),
        Opcode::GeSI64 => Ok(WasmInstruction::GeSI64),
        Opcode::GeUI64 => Ok(WasmInstruction::GeUI64),
        Opcode::LtF32 => Ok(WasmInstruction::LtF32),
        Opcode::GtF32 => Ok(WasmInstruction::GtF32),
        Opcode::LeF32 => Ok(WasmInstruction::LeF32),
        Opcode::GeF32 => Ok(WasmInstruction::GeF32),
        Opcode::LtF64 => Ok(WasmInstruction::LtF64),
        Opcode::GtF64 => Ok(WasmInstruction::GtF64),
        Opcode::LeF64 => Ok(WasmInstruction::LeF64),
        Opcode::GeF64 => Ok(WasmInstruction::GeF64),
        Opcode::RemSI32 => Ok(WasmInstruction::RemSI32),
        Opcode::RemUI32 => Ok(WasmInstruction::RemUI32),
        Opcode::RemSI64 => Ok(WasmInstruction::RemSI64),
        Opcode::RemUI64 => Ok(WasmInstruction::RemUI64),
        Opcode::XorI32 => Ok(WasmInstruction::XorI32),
        Opcode::XorI64 => Ok(WasmInstruction::XorI64),
        Opcode::TruncF32 => todo!(),
        Opcode::NearestF32 => todo!(),
        Opcode::DivF32 => todo!(),
        Opcode::MinF32 => todo!(),
        Opcode::MaxF32 => todo!(),
        Opcode::TruncF64 => todo!(),
        Opcode::NearestF64 => todo!(),
        Opcode::DivF64 => todo!(),
        Opcode::MinF64 => todo!(),
        Opcode::MaxF64 => todo!(),
        Opcode::TruncSI32F32 => todo!(),
        Opcode::TruncUI32F32 => todo!(),
        Opcode::TruncSI32F64 => todo!(),
        Opcode::TruncUI32F64 => todo!(),
        Opcode::ExtendSI64I32 => todo!(),
        Opcode::ExtendUI64I32 => todo!(),
        Opcode::TruncSI64F32 => todo!(),
        Opcode::TruncUI64F32 => todo!(),
        Opcode::TruncSI64F64 => todo!(),
        Opcode::TruncUI64F64 => todo!(),
        Opcode::ConvertSI32F32 => todo!(),
        Opcode::ConvertUI32F32 => todo!(),
        Opcode::ConvertSI64F32 => todo!(),
        Opcode::ConvertUI64F32 => todo!(),
        Opcode::DemoteF32F64 => todo!(),
        Opcode::ConvertSI32F64 => todo!(),
        Opcode::ConvertUI32F64 => todo!(),
        Opcode::ConvertSI64F64 => todo!(),
        Opcode::ConvertUI64F64 => todo!(),
        Opcode::DataDrop => todo!(),
        Opcode::ElemDrop => todo!(),
        Opcode::LoadV128 => todo!(),
        Opcode::StoreV128 => todo!(),
        Opcode::SplatI8x16 => todo!(),
        Opcode::SplatI16x8 => todo!(),
        Opcode::SplatI32x4 => todo!(),
        Opcode::SplatI64x2 => todo!(),
        Opcode::SplatF32x4 => todo!(),
        Opcode::SplatF64x2 => todo!(),
        Opcode::ExtractLaneSI8x16 => todo!(),
        Opcode::ExtractLaneUI8x16 => todo!(),
        Opcode::ExtractLaneSI16x8 => todo!(),
        Opcode::ExtractLaneUI16x8 => todo!(),
        Opcode::ExtractLaneI32x4 => todo!(),
        Opcode::ExtractLaneI64x2 => todo!(),
        Opcode::ExtractLaneF32x4 => todo!(),
        Opcode::ExtractLaneF64x2 => todo!(),
        Opcode::ReplaceLaneI8x16 => todo!(),
        Opcode::ReplaceLaneI16x8 => todo!(),
        Opcode::ReplaceLaneI32x4 => todo!(),
        Opcode::ReplaceLaneI64x2 => todo!(),
        Opcode::ReplaceLaneF32x4 => todo!(),
        Opcode::ReplaceLaneF64x2 => todo!(),
    }
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
                    WasmInstruction::ConstI32(4),
                    WasmInstruction::StoreI32 {
                        flags: 2,
                        offset: 4,
                    },
                    WasmInstruction::End,
                ]),
            }]);
    }
}