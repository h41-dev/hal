use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;

use hal_core::module::{Export, Function, FunctionAddress, FunctionLocal, Memory, MemoryAddress, Value, ValueType};
use hal_core::Trap;

use crate::Result;
use crate::stack::{CallFrame, Stack, StackAccess};
use crate::Store;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Process {
    pub(crate) state: Store,
    pub(crate) stack: Stack,
}


impl Process {
    pub fn new(state: Store) -> Self {
        Self {
            state,
            stack: Stack::default(),
        }
    }

    pub fn function(&self, addr: FunctionAddress) -> core::result::Result<Rc<Function>, Trap> {
        self.state.function(addr)
    }

    pub fn export(&self, name: impl Into<String>) -> core::result::Result<Rc<Export>, Trap> {
        self.export(name)
    }

    pub fn memory(&self, addr: MemoryAddress) -> core::result::Result<Rc<Memory>, Trap> {
        self.state.memory(addr)
    }

    pub(crate) fn unary<T, F>(&mut self, op: F) -> Result<()>
        where
            T: StackAccess,
            F: FnOnce(T) -> T,
    {
        let result = op(self.stack.pop()?);
        self.stack.push(result)
    }

    pub(crate) fn binary<T, F>(&mut self, op: F) -> Result<()>
        where
            T: StackAccess,
            F: FnOnce(T, T) -> T,
    {
        let l = self.stack.pop()?;
        let r = self.stack.pop()?;
        self.stack.push(op(l, r))
    }

    pub(crate) fn binary_trap<T, F>(&mut self, op: F) -> Result<()>
        where
            T: StackAccess,
            F: FnOnce(T, T) -> Result<T>,
    {
        let l = self.stack.pop()?;
        let r = self.stack.pop()?;
        self.stack.push(op(l, r)?)
    }

    pub(crate) fn unary_test<T, F>(&mut self, op: F) -> Result<()>
        where
            T: StackAccess,
            F: FnOnce(T) -> bool,
    {
        let result = op(self.stack.pop()?);
        self.stack.push(if result { Value::I32(1) } else { Value::I32(0) })
    }

    pub(crate) fn binary_test<T, F>(&mut self, op: F) -> Result<()>
        where
            T: StackAccess,
            F: FnOnce(T, T) -> bool,
    {
        let l = self.stack.pop()?;
        let r = self.stack.pop()?;
        let result = op(l, r);
        self.stack.push(if result { Value::I32(1) } else { Value::I32(0) })
    }

    pub(crate) fn push_frame(&mut self, func: &FunctionLocal) -> Result<CallFrame> {
        let mut locals = Vec::with_capacity(func.parameter_count());

        for _ in func.parameters().iter() {
            locals.push(self.stack.pop()?);
        }


        for local in func.locals().iter() {
            match local {
                ValueType::I32 => locals.push(Value::I32(0)),
                ValueType::I64 => locals.push(Value::I64(0)),
            }
        }

        let arity = func.result_count();

        let frame = CallFrame {
            ip: -1,
            sp: self.stack.len(),
            instructions: func.instructions().clone(),
            arity,
            locals: locals.into(),
        };

        Ok(self.stack.replace_frame(frame))
    }


    pub(crate) fn restore_frame(&mut self, frame: CallFrame) {
        let _ = self.stack.restore(frame);
    }
}
