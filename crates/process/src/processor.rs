use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use core::ops::{BitAnd, BitOr, BitXor};

use hal_core::{module, Trap, TrapNotFound, TrapNotImplemented};
use hal_core::module::{ExportData, Function, Instruction, MemoryAddress, Value};
use hal_core::module::FunctionAddress;
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

            Instruction::Call(addr) => {
                let function = process.state.function(addr).unwrap();
                match &*function {
                    Function::Local(local) => invoke_internal(process, self, local)?
                };
            }
            Instruction::ClzI32 => process.unary(|v: i32| v.leading_zeros() as i32)?,
            Instruction::ConstI32(value) => process.stack.push(Value::I32(value))?,
            Instruction::ConstI64(value) => process.stack.push(Value::I64(value))?,
            Instruction::CtzI32 => process.unary(|v: i32| v.trailing_zeros() as i32)?,

            Instruction::DivSI32 => process.binary_trap(i32::div_checked)?,
            Instruction::DivUI32 => process.binary_trap(u32::div_checked)?,

            Instruction::End => { return Ok(ProcessingState::Return); }
            Instruction::EqI32 => process.binary_test(|l: i32, r| l == r)?,
            Instruction::EqzI32 => process.unary_test(|v: i32| v == 0)?,

            Instruction::GeSI32 => process.binary_test(|l: i32, r| l >= r)?,
            Instruction::GeUI32 => process.binary_test(|l: i32, r| (l as u32) >= r as u32)?,
            Instruction::GtSI32 => process.binary_test(|l: i32, r| l > r)?,
            Instruction::GtUI32 => process.binary_test(|l: i32, r| (l as u32) > r as u32)?,

            Instruction::LeSI32 => process.binary_test(|l: i32, r| l <= r)?,
            Instruction::LeUI32 => process.binary_test(|l: i32, r| (l as u32) <= r as u32)?,
            Instruction::LtSI32 => process.binary_test(|l: i32, r| l < r)?,
            Instruction::LtUI32 => process.binary_test(|l: i32, r| (l as u32) < r as u32)?,

            Instruction::MulI32 => process.binary(i32::wrapping_mul)?,
            Instruction::NeI32 => process.binary_test(|l: i32, r| l != r)?,

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

            Instruction::PopcntI32 => process.unary(|v: i32| v.count_ones() as i32)?,

            Instruction::RemSI32 => process.binary_trap(i32::rem_wrapping)?,
            Instruction::RemUI32 => process.binary_trap(u32::rem_wrapping)?,
            Instruction::RotlI32 => process.binary(|l: i32, r| l.rotate_left(r as u32))?,
            Instruction::RotrI32 => process.binary(|l: i32, r| l.rotate_right(r as u32))?,

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

            Instruction::ShlI32 => process.binary(|l: i32, r| l.wrapping_shl(r as u32))?,
            Instruction::ShrSI32 => process.binary(|l: i32, r| l.wrapping_shr(r as u32))?,
            Instruction::ShrUI32 => process.binary(|l: u32, r| l.wrapping_shr(r))?,
            Instruction::SubI32 => process.binary(i32::wrapping_sub)?,

            Instruction::XorI32 => process.binary(i32::bitxor)?,

            _ => return Err(Trap::NotImplemented(TrapNotImplemented::Instruction(inst.clone())))
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
