use std::fmt::Display;

use super::EnumValue;

/// A single value for a parameter in the options.
#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Bool(bool),
    Integer(i64),
    Float(f64),
    Text(String),
    Enum(EnumValue),
}

impl Value {
    /// Returns true if the stored value is a number, i.e., integer or float.
    pub fn is_number(&self) -> bool {
        match self {
            Value::Integer(_) => true,
            Value::Float(_) => true,
            _ => false,
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<u32> for Value {
    fn from(value: u32) -> Self {
        Self::Integer(value as i64)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value as f64)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<EnumValue> for Value {
    fn from(value: EnumValue) -> Self {
        Self::Enum(value)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(x) => write!(f, "{}", x),
            Value::Integer(x) => write!(f, "{}", x),
            Value::Float(x) => write!(f, "{}", x),
            Value::Text(x) => write!(f, "{}", x),
            Value::Enum(x) => write!(f, "{}", x),
        }
    }
}
