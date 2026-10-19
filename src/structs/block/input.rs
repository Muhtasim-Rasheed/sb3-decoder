//! This module defines the structures and enums for handling input values in blocks.

use std::collections::HashMap;

use crate::{
    decoder::{RawBlock, RawBlockInput},
    error::DecodeError,
    structs::{Block, Value},
};

/// Represents the different types of input values that can be associated with a block.
#[derive(Debug, Clone, PartialEq)]
pub enum InputValue {
    /// A literal value, such as a number or string.
    Literal(Value),

    // A color value.
    Color(u8, u8, u8),

    /// A reference to a broadcast by its name.
    Broadcast(String),

    /// A reference to a variable by its name.
    Variable(String),

    /// A reference to a list by its name.
    List(String),

    /// A nested block, represented by another [`Block`].
    Block(Box<Block>),
}

/// The [`BlockInput`] struct wraps an [`InputValue`].
///
/// [`Block`]s contain a map of input names to [`BlockInput`]s.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockInput {
    /// If the block is a shadow block.
    pub shadow: bool,

    /// The input value, which can be a literal, broadcast, or nested block.
    pub value: InputValue,
}

impl BlockInput {
    /// Creates a new [`BlockInput`] from a [`RawBlockInput`] and the entire map of blocks to
    /// un-flatten the structure.
    pub fn new(
        raw: RawBlockInput,
        blocks: &HashMap<String, RawBlock>,
    ) -> Result<Self, DecodeError> {
        let shadow = match raw.shadow {
            1 | 3 => true,
            2 => false,
            _ => unreachable!("Invalid shadow value: {}", raw.shadow),
        };

        let value = match raw.kind {
            0 => {
                // Nested block
                let block_id = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                InputValue::Block(Box::new(Block::new(block_id, blocks)?))
            }
            4..=8 => {
                // Number
                InputValue::Literal(Value::from(raw.value.clone()))
            }
            9 => {
                // Hex color string
                let hex_str = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                if !hex_str.starts_with('#') || hex_str.len() != 7 {
                    return Err(DecodeError::InvalidData(format!(
                        "Invalid hex color string: {}",
                        hex_str
                    )));
                }
                let r = u8::from_str_radix(&hex_str[1..3], 16).map_err(|_| {
                    DecodeError::InvalidData(format!(
                        "Invalid red component in hex color: {}",
                        hex_str
                    ))
                })?;
                let g = u8::from_str_radix(&hex_str[3..5], 16).map_err(|_| {
                    DecodeError::InvalidData(format!(
                        "Invalid green component in hex color: {}",
                        hex_str
                    ))
                })?;
                let b = u8::from_str_radix(&hex_str[5..7], 16).map_err(|_| {
                    DecodeError::InvalidData(format!(
                        "Invalid blue component in hex color: {}",
                        hex_str
                    ))
                })?;
                InputValue::Color(r, g, b)
            }
            10 => {
                // String
                let string = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                InputValue::Literal(Value::String(string.to_string()))
            }
            11 => {
                // Broadcast
                let broadcast_name = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                InputValue::Broadcast(broadcast_name.to_string())
            }
            12 => {
                // Variable
                let var_name = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                InputValue::Variable(var_name.to_string())
            }
            13 => {
                // List
                let list_name = raw
                    .value
                    .as_str()
                    .ok_or_else(|| DecodeError::InvalidData("Expected a string".to_string()))?;
                InputValue::List(list_name.to_string())
            }
            _ => {
                return Err(DecodeError::InvalidData(format!(
                    "Unknown kind value: {}",
                    raw.kind
                )));
            }
        };

        Ok(BlockInput { shadow, value })
    }
}
