use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use core::ops::ControlFlow;
use core::ops::ControlFlow::Continue;

use hal_core::{module, Trap};
use hal_core::module::{ExportData, Function, Instruction, MemoryAddress, Value};
use hal_core::module::FunctionAddress;
use module::FunctionLocal;

use crate::process::Process;
pub(crate) mod invoke;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Processor {}

impl Default for Processor {
    fn default() -> Self {
        Self {}
    }
}

impl Processor {
    fn execute(&self, process: &mut Process) -> Result<(), Trap> {
        loop {
            let Some(frame) = process.call_stack.last_mut() else {
                break;
            };

            frame.ip += 1;

            let Some(inst) = frame.instructions.get(frame.ip as usize) else {
                break;
            };

            match inst {
                Instruction::LocalGet32(addr) => {
                    let Some(value) = frame.locals.get(*addr as usize) else {
                        panic!("not found local");
                    };
                    process.stack.push(value.clone());
                }
                Instruction::LocalSet32(addr) => {
                    let Some(value) = process.stack.pop() else {
                        panic!("not found value in the stack");
                    };
                    let addr = *addr as usize;
                    frame.locals[addr] = value;
                }
                Instruction::End => {
                    process.stack_unwind().unwrap()
                }
                Instruction::ConstI32(value) => process.stack.push(Value::I32(*value)),
                Instruction::ConstI64(value) => process.stack.push(Value::I64(*value)),
                Instruction::StoreI32 { flags: _, offset } => {
                    let (Some(value), Some(addr)) = (process.stack.pop(), process.stack.pop()) else {
                        panic!("not found any value in the stack");
                    };
                    let addr = Into::<i32>::into(addr) as usize;
                    let offset = (*offset) as usize;
                    let at = addr + offset;
                    let end = at + size_of::<i32>();

                    let memory = process
                        .state
                        .memory(addr as MemoryAddress)
                        .unwrap();

                    let value: i32 = value.into();
                    memory.data.borrow_mut()[at..end].copy_from_slice(&value.to_le_bytes());
                }

                Instruction::AddI32 => {
                    let (Some(right), Some(left)) = (process.stack.pop(), process.stack.pop()) else {
                        panic!("not found any value in the stack");
                    };
                    let result = left + right;
                    process.stack.push(result);
                }
                Instruction::AddI64 => {
                    let (Some(right), Some(left)) = (process.stack.pop(), process.stack.pop()) else {
                        panic!("not found any value in the stack");
                    };
                    let result = left + right;
                    process.stack.push(result);
                }
                Instruction::Invoke(addr) => {
                    let function = process.state.function(*addr).unwrap();
                    let func_inst = match &*function {
                        Function::Local(local) => process.push_frame(local)
                    };
                }
                Instruction::MulI32 => {
                    let (Some(right), Some(left)) = (process.stack.pop(), process.stack.pop()) else {
                        panic!("not found any value in the stack");
                    };
                    let result = left * right;
                    process.stack.push(result);
                }
                Instruction::SubI32 => {
                    let (Some(right), Some(left)) = (process.stack.pop(), process.stack.pop()) else {
                        panic!("not found any value in the stack");
                    };
                    let result = left - right;
                    process.stack.push(result);
                }
                _ => todo!("Instruction {:?} not supported yet", inst)
            }
        }
        Ok(())
    }

    fn try_complete() {}

    #[inline(always)]
    fn next(&self, process: &mut Process) -> ControlFlow<Option<Trap>> {
        Continue(())
    }

    pub fn invoke(&self, process: &mut Process, name: impl Into<String>, args: impl AsRef<[Value]>) -> Result<Box<[Value]>, Trap> {
        let name = name.into();

        let idx = match process.state.export(name.clone())
            .or_else(|_| Err(Trap::NotFoundExportedFunction(name)))
            .unwrap()
            .data()
        {
            ExportData::Function(idx) => *idx as usize,
        };

        for arg in args.as_ref() {
            process.stack.push(arg.clone());
        }

        // let func_inst = process.state.function_ref(idx as FunctionIndex).unwrap();
        //
        // match func_inst {
        //     Function::Local(func) => invoke_internal(process, self, func),
        //     // Function::External(func) => invoke_external(fiber, func.clone())
        // }
        let function = process.state.function(idx as FunctionAddress).unwrap();
        let func_inst = match &*function {
            Function::Local(local) => local
        };

        invoke_internal(process, self, func_inst)
    }
}

fn invoke_internal(process: &mut Process, engine: &Processor, func: &FunctionLocal) -> Result<Box<[Value]>, Trap> {
    let arity = func.result_count();

    process.push_frame(func);

    if let Err(e) = engine.execute(process) {
        // self.cleanup();
        panic!("failed to execute instructions: {:?}", e)
    };

    // if arity > 0 {
    //     let Some(value) = process.stack.pop() else {
    //         panic!("not found return value")
    //     };
    //     return Ok(Some(value));
    // }
    // Ok(None)

    let mut result = vec![];

    for _ in 0..arity {
        let Some(value) = process.stack.pop() else {
            panic!("not found return value")
        };
        result.push(value);
    }

    Ok(result.into())
}
