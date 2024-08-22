use alloc::boxed::Box;
use core::fmt::{Display, Formatter};

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ValueType {
    I32,
    I64,
}

impl Display for ValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            ValueType::I32 => write!(f, "i32"),
            ValueType::I64 => write!(f, "i64")
        }
    }
}

impl ValueType {
    pub fn to_str(&self) -> &'static str {
        match self {
            ValueType::I32 => "i32",
            ValueType::I64 => "i64"
        }
    }
}

pub type ValueTypes = Box<[ValueType]>;

#[cfg_attr(any(test, debug_assertions), derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    I64(i64),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Value::I32(v) => write!(f, "{}", v),
            Value::I64(v) => write!(f, "{}", v)
        }
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Value::I32(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}

impl core::ops::Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left.wrapping_add(right)),
            (Value::I64(left), Value::I64(right)) => Value::I64(left.wrapping_add(right)),
            _ => panic!("type mismatch"),
        }
    }
}

impl core::ops::Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left.wrapping_mul(right)),
            (Value::I64(left), Value::I64(right)) => Value::I64(left.wrapping_mul(right)),
            _ => panic!("type mismatch"),
        }
    }
}

impl core::ops::Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::I32(left), Value::I32(right)) => Value::I32(left.wrapping_sub(right)),
            (Value::I64(left), Value::I64(right)) => Value::I64(left.wrapping_sub(right)),
            _ => panic!("type mismatch"),
        }
    }
}

impl From<Value> for i32 {
    fn from(value: Value) -> Self {
        match value {
            Value::I32(value) => value,
            _ => panic!("type mismatch"),
        }
    }
}

impl From<Value> for i64 {
    fn from(value: Value) -> Self {
        match value {
            Value::I64(value) => value,
            _ => panic!("type mismatch"),
        }
    }
}
