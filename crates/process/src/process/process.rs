use alloc::vec::Vec;
use hal_core::module::{Value, ValueType};
use crate::process::frame::Frame;
use crate::process::state::ProcessState;
use crate::Trap;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Process{
    pub state: ProcessState,
    pub stack: Vec<Value>,
    pub call_stack: Vec<Frame>,
}

impl Process {

    // pub(crate) fn push_frame(&mut self, func: &hal_core::module::InternalFunction) {
    //     let bottom = self.stack.len() - func.func_type.params.len();
    //     let mut locals = self.stack.split_off(bottom);
    //
    //     for local in func.code.locals.iter() {
    //         match local {
    //             ValueType::I32 => locals.push(Value::I32(0)),
    //             ValueType::I64 => locals.push(Value::I64(0)),
    //         }
    //     }
    //
    //     let arity = func.func_type.results.len();
    //
    //     let frame = Frame {
    //         ip: -1,
    //         sp: self.stack.len(),
    //         instructions: func.code.body.clone(),
    //         arity,
    //         locals,
    //     };
    //
    //     self.call_stack.push(frame);
    // }

    #[inline(always)]
    pub fn stack_unwind(&mut self) -> Result<(), Trap> {
        let Some(frame) = self.call_stack.pop() else {
            panic!("not found frame");
        };
        let Frame { sp, arity, .. } = frame;
        let stack: &mut Vec<Value> = self.stack.as_mut();

        if arity > 0 {
            let Some(value) = stack.pop() else {
                // return Err(ProcessorError::NotFoundReturnValue);
                todo!() // FIXME
            };
            stack.drain(sp..);
            stack.push(value);
        } else {
            stack.drain(sp..);
        }
        Ok(())
    }
}
