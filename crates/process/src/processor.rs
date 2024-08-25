use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use core::ops::{BitAnd, BitOr, BitXor};

use hal_core::{module, Trap, TrapNotFound, TrapNotImplemented};
use hal_core::module::{ExportData, Function, Instruction, MemoryAddress, Value};
use hal_core::module::FunctionAddress;
use hal_core::module::ValueType::I32;
use module::FunctionLocal;

use crate::numeric::Integer;
use crate::process::Process;

pub enum ProcessingState {
    Break,
    Continue,
    Return,
}

type ProcessorResult = core::result::Result<ProcessingState, Trap>;

// Processor might own processes
#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Processor {}


impl Default for Processor {
    fn default() -> Self {
        Self {}
    }
}

impl Processor {
    fn until_completion(&self, process: &mut Process) -> Result<(), Trap> {
        loop {
            match self.next(process) {
                Ok(state) => {
                    match state {
                        ProcessingState::Break => todo!(),
                        ProcessingState::Continue => continue,
                        ProcessingState::Return => return Ok(())
                    }
                }
                Err(t) => return Err(t)
            };
        }
    }

    fn try_complete() {}

    fn next(&self, process: &mut Process) -> ProcessorResult {
        let stack = &mut process.stack;
        stack.frame.ip += 1;

        let inst = { stack.frame.instructions.get(stack.frame.ip as usize).unwrap().clone() };

        match inst {
            Instruction::AddI32 => process.binary(i32::wrapping_add)?,
            Instruction::AddI64 => process.binary(i64::wrapping_add)?,
            Instruction::AndI32 => process.binary(i32::bitand)?,
            Instruction::AndI64 => process.binary(i64::bitand)?,

            Instruction::Call(addr) => {
                let function = process.state.function(addr).unwrap();
                match &*function {
                    Function::Local(local) => invoke_internal(process, self, local)?
                };
            }
            Instruction::ClzI32 => process.unary(|v: i32| v.leading_zeros() as i32)?,
            Instruction::ClzI64 => process.unary(|v: i64| v.leading_zeros() as i64)?,

            Instruction::ConstI32(value) => process.stack.push(Value::I32(value))?,
            Instruction::ConstI64(value) => process.stack.push(Value::I64(value))?,

            Instruction::CtzI32 => process.unary(|v: i32| v.trailing_zeros() as i32)?,
            Instruction::CtzI64 => process.unary(|v: i64| v.trailing_zeros() as i64)?,

            Instruction::DivSI32 => process.binary_trap(i32::div_checked)?,
            Instruction::DivSI64 => process.binary_trap(i64::div_checked)?,

            Instruction::DivUI32 => process.binary_trap(u32::div_checked)?,
            Instruction::DivUI64 => process.binary_trap(u64::div_checked)?,

            Instruction::End => { return Ok(ProcessingState::Return); }

            Instruction::EqI32 => process.binary_test(|l: i32, r| l == r)?,
            Instruction::EqI64 => process.binary_test(|l: i64, r| l == r)?,

            Instruction::EqzI32 => process.unary_test(|v: i32| v == 0)?,
            Instruction::EqzI64 => process.unary_test(|v: i64| v == 0)?,

            Instruction::Extend8SI32 => process.unary_map(|v: i32| i32::from(v as i8))?,
            Instruction::Extend8SI64 => process.unary_map(|v: i64| i64::from(v as i8))?,
            Instruction::Extend16SI32 => process.unary_map(|v: i32| i32::from(v as i16))?,
            Instruction::Extend16SI64 => process.unary_map(|v: i64| i64::from(v as i16))?,
            Instruction::Extend32SI64 => process.unary_map(|v: i64| i64::from(v as i32))?,

            Instruction::GeSI32 => process.binary_test(|l: i32, r| l >= r)?,
            Instruction::GeSI64 => process.binary_test(|l: i64, r| l >= r)?,

            Instruction::GeUI32 => process.binary_test(|l: i32, r| (l as u32) >= r as u32)?,
            Instruction::GeUI64 => process.binary_test(|l: i64, r| (l as u64) >= r as u64)?,

            Instruction::GtSI32 => process.binary_test(|l: i32, r| l > r)?,
            Instruction::GtSI64 => process.binary_test(|l: i64, r| l > r)?,

            Instruction::GtUI32 => process.binary_test(|l: i32, r| (l as u32) > r as u32)?,
            Instruction::GtUI64 => process.binary_test(|l: i64, r| (l as u64) > r as u64)?,

            Instruction::LeSI32 => process.binary_test(|l: i32, r| l <= r)?,
            Instruction::LeSI64 => process.binary_test(|l: i64, r| l <= r)?,

            Instruction::LeUI32 => process.binary_test(|l: i32, r| (l as u32) <= r as u32)?,
            Instruction::LeUI64 => process.binary_test(|l: i64, r| (l as u64) <= r as u64)?,

            Instruction::LtSI32 => process.binary_test(|l: i32, r| l < r)?,
            Instruction::LtSI64 => process.binary_test(|l: i64, r| l < r)?,

            Instruction::LtUI32 => process.binary_test(|l: i32, r| (l as u32) < r as u32)?,
            Instruction::LtUI64 => process.binary_test(|l: i64, r| (l as u64) < r as u64)?,

            Instruction::MulI32 => process.binary(i32::wrapping_mul)?,
            Instruction::MulI64 => process.binary(i64::wrapping_mul)?,

            Instruction::NeI32 => process.binary_test(|l: i32, r| l != r)?,
            Instruction::NeI64 => process.binary_test(|l: i64, r| l != r)?,

            Instruction::LocalGet32(addr) => {
                let Some(value) = stack.frame.locals.get(addr as usize) else {
                    panic!("not found local");
                };
                stack.push(value.clone())?;
            }
            Instruction::LocalSet32(addr) => {
                let value = stack.pop()?;
                let addr = addr as usize;
                stack.frame.locals[addr] = value;
            }

            Instruction::OrI32 => process.binary(i32::bitor)?,
            Instruction::OrI64 => process.binary(i64::bitor)?,

            Instruction::PopcntI32 => process.unary(|v: i32| v.count_ones() as i32)?,
            Instruction::PopcntI64 => process.unary(|v: i64| v.count_ones() as i64)?,

            Instruction::RemSI32 => process.binary_trap(i32::rem_wrapping)?,
            Instruction::RemSI64 => process.binary_trap(i64::rem_wrapping)?,

            Instruction::RemUI32 => process.binary_trap(u32::rem_wrapping)?,
            Instruction::RemUI64 => process.binary_trap(u64::rem_wrapping)?,

            Instruction::RotlI32 => process.binary(|l: i32, r| l.rotate_left(r as u32))?,
            Instruction::RotlI64 => process.binary(|l: i64, r| l.rotate_left(r as u32))?,

            Instruction::RotrI32 => process.binary(|l: i32, r| l.rotate_right(r as u32))?,
            Instruction::RotrI64 => process.binary(|l: i64, r| l.rotate_right(r as u32))?,

            Instruction::StoreI32 { flags: _, offset } => {
                let (value, addr): (Value, Value) = (process.stack.pop()?, process.stack.pop()?);

                let addr = Into::<i32>::into(addr) as usize;
                let offset = (offset) as usize;
                let at = addr + offset;
                let end = at + size_of::<i32>();

                let memory = process
                    .state
                    .memory(addr as MemoryAddress)
                    .unwrap();

                let value: i32 = value.into();
                memory.data.borrow_mut()[at..end].copy_from_slice(&value.to_le_bytes());
            }
            Instruction::StoreI64 { flags: _, offset } => {
                let (value, addr): (Value, Value) = (process.stack.pop()?, process.stack.pop()?);

                let addr = Into::<i64>::into(addr) as usize;
                let offset = (offset) as usize;
                let at = addr + offset;
                let end = at + size_of::<i64>();

                let memory = process
                    .state
                    .memory(addr as MemoryAddress)
                    .unwrap();

                let value: i64 = value.into();
                memory.data.borrow_mut()[at..end].copy_from_slice(&value.to_le_bytes());
            }

            Instruction::ShlI32 => process.binary(|l: i32, r| l.wrapping_shl(r as u32))?,
            Instruction::ShlI64 => process.binary(|l: i64, r| l.wrapping_shl(r as u32))?,

            Instruction::ShrSI32 => process.binary(|l: i32, r| l.wrapping_shr(r as u32))?,
            Instruction::ShrSI64 => process.binary(|l: i64, r| l.wrapping_shr(r as u32))?,

            Instruction::ShrUI32 => process.binary(|l: u32, r| l.wrapping_shr(r))?,
            Instruction::ShrUI64 => process.binary(|l: u64, r| l.wrapping_shr(r as u32))?,

            Instruction::SubI32 => process.binary(i32::wrapping_sub)?,
            Instruction::SubI64 => process.binary(i64::wrapping_sub)?,

            Instruction::XorI32 => process.binary(i32::bitxor)?,
            Instruction::XorI64 => process.binary(i64::bitxor)?,

            _ => return Err(Trap::NotImplemented(TrapNotImplemented::Instruction(inst.clone()))),
        }

        return Ok(ProcessingState::Continue);
    }

    pub fn invoke(&self, process: &mut Process, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Box<[Value]>, Trap> {
        let name = name.into();

        let idx = match process.state.export(name.clone())
            .or_else(|_| Err(Trap::NotFound(TrapNotFound::ExportedFunction(name))))
            .unwrap()
            .data()
        {
            ExportData::Function(idx) => *idx as usize,
        };

        for arg in args.as_ref() {
            process.stack.push(arg.clone())?;
        }

        let function = process.state.function(idx as FunctionAddress).unwrap();
        let func_inst = match &*function {
            Function::Local(local) => local
        };

        invoke_internal(process, self, func_inst)?;

        let mut result = vec![];
        for _ in 0..func_inst.result_count() {
            let value = process.stack.pop()?;
            result.push(value);
        }
        Ok(result.into())
    }
}


fn invoke_internal(process: &mut Process, processor: &Processor, func: &FunctionLocal) -> Result<(), Trap> {
    let previous_frame = process.push_frame(func)?;

    if let Err(e) = processor.until_completion(process) {
        // self.cleanup();
        // FIXME
        return Err(e);
    };

    process.restore_frame(previous_frame);
    Ok(())
}
