use alloc::fmt;
use alloc::vec::Vec;

use hal_core::{Trap, TrapOverflow, TrapType, TrapUnderflow};
use hal_core::module::{Value, ValueType};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct CallFrame {}

impl Default for CallFrame {
    fn default() -> Self {
        Self {}
    }
}

pub(crate) const MAX_STACK_32_SIZE: usize = 1024 * 32;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
pub struct Stack {
    // byte buffer of values
    bytes: Vec<u8>,
    types: Vec<ValueType>,
    frame: CallFrame,
}

type Result<T> = core::result::Result<T, Trap>;

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
        match stack.top_type()? {
            ValueType::I32 => StackAccess::pop(stack).map(|v| Value::I32(v)),
            ValueType::I64 => StackAccess::pop(stack).map(|v| Value::I64(v)),
        }
    }

    fn peek(stack: &Stack) -> Result<Self> {
        match stack.top_type()? {
            ValueType::I32 => StackAccess::peek(stack).map(|v| Value::I32(v)),
            ValueType::I64 => StackAccess::peek(stack).map(|v| Value::I64(v)),
        }
    }
}


impl Stack {
    pub fn push<V: StackAccess>(&mut self, v: V) -> Result<()> {
        StackAccess::push(self, v)
    }

    pub fn peek<V: StackAccess>(&mut self) -> Result<V> {
        StackAccess::peek(self)
    }

    pub fn pop<V: StackAccess>(&mut self) -> Result<V> {
        StackAccess::pop(self)
    }

    fn push_bytes(&mut self, bytes: &[u8], vt: ValueType) -> Result<()> {
        if self.types.len() + 1 > MAX_STACK_32_SIZE {
            return Err(Trap::Overflow(TrapOverflow::Stack));
        }
        self.bytes.extend_from_slice(bytes);
        self.types.push(vt);
        Ok(())
    }

    fn pop_bytes(&mut self, s: usize) -> Result<()> {
        self.bytes.truncate(self.bytes.len() - s);
        self.types.pop().ok_or(Trap::Underflow(TrapUnderflow::Stack))?;
        Ok(())
    }

    fn peek_bytes<'a, T>(&'a self, s: usize) -> Result<T>
        where
            T: TryFrom<&'a [u8]>,
            T::Error: fmt::Debug,
    {
        Ok(self.bytes[self.bytes.len() - s..].try_into()
            .map_err(|_| Trap::Underflow(TrapUnderflow::Stack))?)
    }

    fn expect_type(&self, expected: ValueType) -> Result<()> {
        let got = self.top_type()?.clone();
        if got != expected {
            Err(Trap::Type(TrapType::Mismatch(expected, got)))
        } else {
            Ok(())
        }
    }

    fn top_type(&self) -> Result<&ValueType> {
        self.types.last().ok_or(Trap::Underflow(TrapUnderflow::Stack))
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
        for i in 0..MAX_STACK_32_SIZE  {
            ti.push(i as i32).unwrap()
        }

        let result: Result<()> = ti.push(42i32);
        assert_eq!(
            result,
            Err(Trap::Overflow(TrapOverflow::Stack))
        );
    }
}