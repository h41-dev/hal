use alloc::boxed::Box;
use alloc::fmt;
use alloc::vec::Vec;
use core::mem;

use hal_core::{Trap, TrapOverflow, TrapType, TrapUnderflow};
use hal_core::module::{Instruction, Value, ValueType};

use crate::Result;

pub type InstructionPointer = isize;
pub type StackPointer = usize;
pub type Arity = usize;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct CallFrame {
    pub(crate) ip: InstructionPointer,
    pub(crate) sp: StackPointer,
    pub(crate) instructions: Box<[Instruction]>,
    pub(crate) arity: Arity,
    pub(crate) locals: Box<[Value]>,
}

impl Default for CallFrame {
    fn default() -> Self {
        Self {
            ip: 0,
            sp: 0,
            instructions: Box::new([]),
            arity: 0,
            locals: Box::new([]),
        }
    }
}

pub(crate) const MAX_VALUE_STACK: usize = 1024 * 32;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Stack {
    bytes: Vec<u8>,
    types: Vec<ValueType>,
    pub(crate) frame: CallFrame,
}

/// A trait that defines stack operations for a specific type.
/// This trait is intended to be implemented by types that can be pushed, popped, and peeked on a stack.
///
/// # Type Parameter
/// - `Self`: The type of the value that will be manipulated on the stack.
///
/// # Errors
/// Each method returns a `Result` to handle potential errors:
/// - `Ok`: The operation was successful.
/// - `Trap`: The operation trapped due to an error such as a stack overflow, stack underflow, or type mismatch.
pub trait StackAccess: Sized {
    /// Pushes a value onto the stack and returns the current stack height.
    ///
    /// # Parameters
    /// - `stack`: A mutable reference to the `Stack` where the value will be pushed.
    /// - `value`: The value to be pushed onto the stack.
    ///
    /// # Returns
    /// - `Result<()>`:
    /// - `Trap`: Traps if there is a stack overflow.
    fn push(stack: &mut Stack, value: Self) -> Result<()>;

    /// Pops a value from the stack.
    ///
    /// # Parameters
    /// - `stack`: A mutable reference to the `Stack` from which the value will be popped.
    ///
    /// # Returns
    /// - `Result<Self>`: Returns `Ok(Self)` if the value was successfully popped from the stack.
    /// - `Trap`: Traps if there is a stack underflow or if the value type does not match the expected type.
    fn pop(stack: &mut Stack) -> Result<Self> {
        let value = Self::peek(stack)?;
        stack.pop_bytes(size_of::<Self>())?;
        Ok(value)
    }

    /// Peeks at the top value of the stack without removing it.
    ///
    /// # Parameters
    /// - `stack`: A reference to the `Stack` from which the value will be peeked.
    ///
    /// # Returns
    /// - `Result<Self>`: Returns `Ok(Self)` if the value was successfully peeked at the top of the stack.
    /// - `Trap`: Traps if the value type does not match the expected type.
    fn peek(stack: &Stack) -> Result<Self>;
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            bytes: Default::default(),
            types: Default::default(),
            frame: Default::default(),
        }
    }
}

impl StackAccess for i32 {
    fn push(stack: &mut Stack, value: Self) -> Result<()> {
        stack.push_bytes(&value.to_le_bytes(), ValueType::I32)
    }

    fn peek(stack: &Stack) -> Result<Self> {
        stack.expect_type(ValueType::I32)?;
        Ok(Self::from_le_bytes(stack.peek_bytes(size_of::<Self>())?))
    }
}

impl StackAccess for u32 {
    fn push(stack: &mut Stack, value: Self) -> Result<()> {
        stack.push_bytes(&value.to_le_bytes(), ValueType::I32)
    }

    fn peek(stack: &Stack) -> Result<Self> {
        stack.expect_type(ValueType::I32)?;
        Ok(u32::from_le_bytes(stack.peek_bytes(size_of::<i32>())?))
    }
}

impl StackAccess for i64 {
    fn push(stack: &mut Stack, value: Self) -> Result<()> {
        stack.push_bytes(&value.to_le_bytes(), ValueType::I64)
    }

    fn peek(stack: &Stack) -> Result<Self> {
        stack.expect_type(ValueType::I64)?;
        Ok(Self::from_le_bytes(stack.peek_bytes(size_of::<Self>())?))
    }
}

impl StackAccess for Value {
    fn push(stack: &mut Stack, v: Self) -> Result<()> {
        match v {
            Value::I32(i) => StackAccess::push(stack, i),
            Value::I64(i) => StackAccess::push(stack, i),
        }
    }
    fn pop(stack: &mut Stack) -> Result<Self> {
        match stack.peek_type()? {
            ValueType::I32 => StackAccess::pop(stack).map(|v| Value::I32(v)),
            ValueType::I64 => StackAccess::pop(stack).map(|v| Value::I64(v)),
        }
    }

    fn peek(stack: &Stack) -> Result<Self> {
        match stack.peek_type()? {
            ValueType::I32 => StackAccess::peek(stack).map(|v| Value::I32(v)),
            ValueType::I64 => StackAccess::peek(stack).map(|v| Value::I64(v)),
        }
    }
}


impl Stack {
    /// Pushes a value onto the stack.
    ///
    /// This function accepts any type that implements the `StackAccess` trait and pushes
    /// it onto the stack. The actual pushing logic is delegated to the `StackAccess::push`
    /// method, which handles the specific behavior for the value type.
    ///
    /// # Parameters
    ///
    /// - `v`: The value to be pushed onto the stack.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` on success, or an error if the push operation fails.
    pub fn push<V: StackAccess>(&mut self, v: V) -> Result<()> {
        StackAccess::push(self, v)
    }

    /// Peeks at the top value of the stack without removing it.
    ///
    /// This function retrieves the top value from the stack without modifying the stack's
    /// state. The value is returned as a result, and the specific behavior depends on the
    /// implementation of the `StackAccess::peek` method.
    ///
    /// # Returns
    ///
    /// - `Result<V>`: The top value on the stack, or an error if the peek operation fails.
    pub fn peek<V: StackAccess>(&mut self) -> Result<V> {
        StackAccess::peek(self)
    }

    /// Pops the top value off the stack.
    ///
    /// This function removes and returns the top value from the stack. The actual pop logic
    /// is handled by the `StackAccess::pop` method, which manages the specific behavior for
    /// the value type.
    ///
    /// # Returns
    ///
    /// - `Result<V>`: The value that was popped from the stack, or an error if the pop operation fails.
    pub fn pop<V: StackAccess>(&mut self) -> Result<V> {
        StackAccess::pop(self)
    }

    /// Pushes raw bytes onto the stack along with a specified value type.
    ///
    /// This internal function directly manipulates the stack by pushing a slice of bytes and
    /// a corresponding `ValueType`. It checks for stack overflow before performing the push
    /// operation.
    ///
    /// # Parameters
    ///
    /// - `bytes`: A slice of bytes to push onto the stack.
    /// - `vt`: The `ValueType` associated with the bytes.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` on success, or an error if the stack overflows.
    fn push_bytes(&mut self, bytes: &[u8], vt: ValueType) -> Result<()> {
        if self.types.len() + 1 > MAX_VALUE_STACK {
            return Err(Trap::Overflow(TrapOverflow::Stack));
        }
        self.bytes.extend_from_slice(bytes);
        self.types.push(vt);
        Ok(())
    }

    /// Pops raw bytes off the stack based on the specified size.
    ///
    /// This internal function removes a specified number of bytes from the top of the stack
    /// and adjusts the stack's state accordingly. It checks for stack underflow before performing
    /// the pop operation.
    ///
    /// # Parameters
    ///
    /// - `s`: The number of bytes to remove from the stack.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` on success, or an error if the stack underflows.
    fn pop_bytes(&mut self, s: usize) -> Result<()> {
        self.bytes.truncate(self.bytes.len() - s);
        self.types.pop().ok_or(Trap::Underflow(TrapUnderflow::Stack))?;
        Ok(())
    }

    /// Peeks at a specific number of bytes from the top of the stack.
    ///
    /// This internal function retrieves a slice of bytes from the top of the stack without
    /// removing them. The bytes are converted to the specified type `T`, which must implement
    /// the `TryFrom` trait.
    ///
    /// # Parameters
    ///
    /// - `s`: The number of bytes to peek at the top of the stack.
    ///
    /// # Returns
    ///
    /// - `Result<T>`: The converted value from the bytes, or an error if the conversion fails or the stack underflows.
    fn peek_bytes<'a, T>(&'a self, s: usize) -> Result<T>
        where
            T: TryFrom<&'a [u8]>,
            T::Error: fmt::Debug,
    {
        Ok(self.bytes[self.bytes.len() - s..].try_into()
            .map_err(|_| Trap::Underflow(TrapUnderflow::Stack))?)
    }

    /// Checks if the top value on the stack matches the expected type.
    ///
    /// This internal function compares the `ValueType` of the top value on the stack with
    /// the expected type. If the types do not match, an error is returned.
    ///
    /// # Parameters
    ///
    /// - `expected`: The expected `ValueType` for the top value on the stack.
    ///
    /// # Returns
    ///
    /// - `Result<()>`: Returns `Ok(())` if the types match, or an error if they do not.
    fn expect_type(&self, expected: ValueType) -> Result<()> {
        let got = self.peek_type()?.clone();
        if got != expected {
            Err(Trap::Type(TrapType::Mismatch(expected, got)))
        } else {
            Ok(())
        }
    }

    /// Peeks at the type of the top value on the stack.
    ///
    /// This internal function retrieves the `ValueType` of the top value on the stack without
    /// removing it. This is useful for type-checking operations.
    ///
    /// # Returns
    ///
    /// - `Result<&ValueType>`: The type of the top value on the stack, or an error if the stack is empty.
    fn peek_type(&self) -> Result<&ValueType> {
        self.types.last().ok_or(Trap::Underflow(TrapUnderflow::Stack))
    }

    /// Returns the current number of values on the stack.
    ///
    /// This function provides the current size of the stack, which is determined by the number
    /// of `ValueType` entries.
    ///
    /// # Returns
    ///
    /// - `usize`: The number of values on the stack.
    pub(crate) fn len(&self) -> usize {
        self.types.len()
    }

    /// Replaces the current call frame with a new one and returns the old frame.
    ///
    /// This function takes ownership of a new `CallFrame` and replaces the
    /// current call frame stored within the object. The previous call frame
    /// is returned as a result, allowing it to be further manipulated or
    /// stored if needed.
    ///
    /// # Parameters
    ///
    /// - `frame`: The new `CallFrame` to be set as the current call frame.
    ///
    /// # Returns
    ///
    /// - The previous `CallFrame` that was replaced.
    pub(crate) fn replace_frame(&mut self, frame: CallFrame) -> CallFrame {
        mem::replace(&mut self.frame, frame)
    }

    /// Restores a previous `CallFrame` as the current frame without returning the old one.
    ///
    /// This function replaces the current `CallFrame` with the provided one, effectively
    /// restoring a previous state. Unlike `replace_frame`, this function does not return
    /// the old frame, discarding it instead. This is useful in scenarios where the current
    /// frame needs to be quickly swapped out without the need to retain the old frame.
    ///
    /// # Parameters
    ///
    /// - `frame`: The `CallFrame` to be restored as the current frame.
    pub(crate) fn restore(&mut self, frame: CallFrame) {
        _ = mem::replace(&mut self.frame, frame);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i32_primitive() {
        let mut ti = Stack::default();
        ti.push(0i32).unwrap();
        assert_eq!(ti.peek::<i32>().unwrap(), 0);

        ti.push(1i32).unwrap();
        ti.push(-1i32).unwrap();
        ti.push(i32::MAX).unwrap();
        ti.push(i32::MIN).unwrap();

        assert_eq!(ti.peek::<i32>().unwrap(), i32::MIN);
        assert_eq!(ti.pop::<i32>().unwrap(), i32::MIN);
        assert_eq!(ti.peek::<i32>().unwrap(), i32::MAX);
        assert_eq!(ti.pop::<i32>().unwrap(), i32::MAX);
        assert_eq!(ti.peek::<i32>().unwrap(), -1);
        assert_eq!(ti.pop::<i32>().unwrap(), -1);
        assert_eq!(ti.peek::<i32>().unwrap(), 1);
        assert_eq!(ti.pop::<i32>().unwrap(), 1);
        assert_eq!(ti.peek::<i32>().unwrap(), 0);
        assert_eq!(ti.pop::<i32>().unwrap(), 0);
    }

    #[test]
    fn i32_value() {
        let mut ti = Stack::default();
        ti.push(Value::I32(0i32)).unwrap();
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(0));

        ti.push(Value::I32(1i32)).unwrap();
        ti.push(Value::I32(-1i32)).unwrap();
        ti.push(Value::I32(i32::MAX)).unwrap();
        ti.push(Value::I32(i32::MIN)).unwrap();

        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(i32::MIN));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(i32::MIN));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(i32::MAX));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(i32::MAX));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(-1));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(-1));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(1));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(1));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(0));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(0));
    }

    #[test]
    fn i32_mixed() {
        let mut ti = Stack::default();
        ti.push(Value::I32(i32::MAX)).unwrap();
        ti.push(i32::MAX).unwrap();
        ti.push(Value::I32(i32::MIN)).unwrap();
        ti.push(i32::MIN).unwrap();

        assert_eq!(ti.peek::<i32>().unwrap(), i32::MIN);
        assert_eq!(ti.pop::<i32>().unwrap(), i32::MIN);
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(i32::MIN));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(i32::MIN));
        assert_eq!(ti.peek::<i32>().unwrap(), i32::MAX);
        assert_eq!(ti.pop::<i32>().unwrap(), i32::MAX);
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I32(i32::MAX));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I32(i32::MAX));
    }

    #[test]
    fn i64_primitive() {
        let mut ti = Stack::default();
        ti.push(0i64).unwrap();
        assert_eq!(ti.peek::<i64>().unwrap(), 0);

        ti.push(1i64).unwrap();
        ti.push(-1i64).unwrap();
        ti.push(i64::MAX).unwrap();
        ti.push(i64::MIN).unwrap();

        assert_eq!(ti.peek::<i64>().unwrap(), i64::MIN);
        assert_eq!(ti.pop::<i64>().unwrap(), i64::MIN);
        assert_eq!(ti.peek::<i64>().unwrap(), i64::MAX);
        assert_eq!(ti.pop::<i64>().unwrap(), i64::MAX);
        assert_eq!(ti.peek::<i64>().unwrap(), -1);
        assert_eq!(ti.pop::<i64>().unwrap(), -1);
        assert_eq!(ti.peek::<i64>().unwrap(), 1);
        assert_eq!(ti.pop::<i64>().unwrap(), 1);
        assert_eq!(ti.peek::<i64>().unwrap(), 0);
        assert_eq!(ti.pop::<i64>().unwrap(), 0);
    }

    #[test]
    fn i64_value() {
        let mut ti = Stack::default();
        ti.push(Value::I64(0i64)).unwrap();
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(0));

        ti.push(Value::I64(1i64)).unwrap();
        ti.push(Value::I64(-1i64)).unwrap();
        ti.push(Value::I64(i64::MAX)).unwrap();
        ti.push(Value::I64(i64::MIN)).unwrap();

        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(i64::MIN));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(i64::MIN));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(i64::MAX));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(i64::MAX));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(-1));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(-1));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(1));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(1));
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(0));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(0));
    }

    #[test]
    fn i64_mixed() {
        let mut ti = Stack::default();
        ti.push(Value::I64(i64::MAX)).unwrap();
        ti.push(i64::MAX).unwrap();
        ti.push(Value::I64(i64::MIN)).unwrap();
        ti.push(i64::MIN).unwrap();

        assert_eq!(ti.peek::<i64>().unwrap(), i64::MIN);
        assert_eq!(ti.pop::<i64>().unwrap(), i64::MIN);
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(i64::MIN));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(i64::MIN));
        assert_eq!(ti.peek::<i64>().unwrap(), i64::MAX);
        assert_eq!(ti.pop::<i64>().unwrap(), i64::MAX);
        assert_eq!(ti.peek::<Value>().unwrap(), Value::I64(i64::MAX));
        assert_eq!(ti.pop::<Value>().unwrap(), Value::I64(i64::MAX));
    }

    #[test]
    fn type_mismatch_on_pop() {
        let mut ti = Stack::default();
        ti.push(42i32).unwrap();

        let result: Result<i64> = ti.pop();
        assert_eq!(
            result,
            Err(Trap::Type(TrapType::Mismatch(ValueType::I64, ValueType::I32)))
        );
    }

    #[test]
    fn type_mismatch_on_pop_value() {
        let mut ti = Stack::default();
        ti.push(Value::I32(23)).unwrap();
        let result: Result<i64> = ti.pop();
        assert_eq!(
            result,
            Err(Trap::Type(TrapType::Mismatch(ValueType::I64, ValueType::I32)))
        );
    }

    #[test]
    fn type_mismatch_on_peek() {
        let mut ti = Stack::default();
        ti.push(42i32).unwrap();
        let result: Result<i64> = ti.peek();
        assert_eq!(
            result,
            Err(Trap::Type(TrapType::Mismatch(ValueType::I64, ValueType::I32)))
        );
    }

    #[test]
    fn type_mismatch_on_peek_value() {
        let mut ti = Stack::default();
        ti.push(Value::I32(23)).unwrap();
        let result: Result<i64> = ti.peek();
        assert_eq!(
            result,
            Err(Trap::Type(TrapType::Mismatch(ValueType::I64, ValueType::I32)))
        );
    }

    #[test]
    fn stack_underflow_on_pop() {
        let mut ti = Stack::default();
        let result: Result<i32> = ti.pop();
        assert_eq!(result, Err(Trap::Underflow(TrapUnderflow::Stack)));
    }

    #[test]
    fn stack_underflow_on_peek() {
        let mut ti = Stack::default();
        let result: Result<i32> = ti.peek();
        assert_eq!(result, Err(Trap::Underflow(TrapUnderflow::Stack)));
    }

    #[test]
    fn stack_overflow() {
        let mut ti = Stack::default();
        for i in 0..MAX_VALUE_STACK {
            ti.push(i as i32).unwrap()
        }

        let result: Result<()> = ti.push(42i32);
        assert_eq!(
            result,
            Err(Trap::Overflow(TrapOverflow::Stack))
        );
    }

    #[test]
    fn len() {
        let mut ti = Stack::default();
        assert_eq!(ti.len(), 0);
        ti.push(23i32).unwrap();
        assert_eq!(ti.len(), 1);
        ti.pop::<i32>().unwrap();
        assert_eq!(ti.len(), 0);

        let _ = ti.pop::<i32>();
        let _ = ti.pop::<i32>();

        assert_eq!(ti.len(), 0);
    }
}