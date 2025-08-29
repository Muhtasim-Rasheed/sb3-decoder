//! Module for handling Scratch 3.0 values and variables.
//!
//! This module defines the [`Value`] enum to represent variable values and the [`Variable`] struct to
//! represent variables in a Scratch 3.0 project. It also includes the [`List`] struct for handling
//! lists.

/// The [`Value`] enum represents the possible types of values a variable can hold in a Scratch 3.0
/// project.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Value {
    /// Checks if the value is scratch's representation of null.
    pub fn is_null(&self) -> bool {
        matches!(self, Value::String(s) if s == "null")
    }
}

impl From<serde_json::Value> for Value {
    fn from(value: serde_json::Value) -> Self {
        match value {
            serde_json::Value::Number(num) => {
                if let Some(n) = num.as_f64() {
                    Value::Number(n)
                } else {
                    Value::String(num.to_string())
                }
            }
            serde_json::Value::String(s) => match s.as_str() {
                "Infinity" => Value::Number(f64::INFINITY),
                "-Infinity" => Value::Number(f64::NEG_INFINITY),
                "NaN" => Value::Number(f64::NAN),
                _ => {
                    if let Ok(n) = s.parse::<f64>() {
                        Value::Number(n)
                    } else {
                        Value::String(s)
                    }
                }
            },
            _ => Value::String(value.to_string()),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => match s.to_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err("Cannot convert string to bool"),
            },
            _ => Err("Cannot convert non-string to bool"),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = &'static str;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(n) => Ok(n),
            Value::String(s) => s.parse::<f64>().map_err(|_| "Cannot parse string to f64"),
        }
    }
}

/// The [`Variable`] struct represents a variable in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub struct Variable(
    /// The value of the variable.
    pub Value,
);

/// The [`List`] struct represents a list in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub struct List(
    /// The items in the list.
    pub Vec<Value>,
);
