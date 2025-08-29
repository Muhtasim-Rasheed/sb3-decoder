//! This module provides the [`Block`] struct. It represents a Scratch block inside a target.

use std::collections::HashMap;

use crate::{decoder::RawBlock, error::DecodeError, structs::{BlockInput, Opcode}};

/// The [`Block`] struct represents a Scratch block inside a target. It has an opcode, some inputs,
/// some fields, and thats about it.
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The opcode of the block.
    pub opcode: Opcode,

    /// The inputs of the block, represented as a map from input name to a [`BlockInput`].
    pub inputs: HashMap<String, BlockInput>,

    /// The fields of the block, represented as a map from field name to a variable name.
    pub fields: HashMap<String, String>,
}

impl Block {
    /// Creates a new empty block with the given block ID and the entire [`crate::decoder::RawSprite::blocks`]
    /// map to un-flatten the structure.
    pub fn new(id: &str, blocks: &HashMap<String, RawBlock>) -> Result<Self, DecodeError> {
        let raw = blocks
            .get(id)
            .ok_or_else(|| DecodeError::NotFound(format!("Block with ID {}", id)))?;
        let inputs = raw
            .inputs
            .iter()
            .map(|(k, v)| {
                Ok((
                    k.clone(),
                    BlockInput::new(v.clone(), blocks)?,
                ))
            })
            .collect::<Result<HashMap<String, BlockInput>, DecodeError>>()?;
        let fields = raw
            .fields
            .iter()
            .map(|(f, (name, _))| (f.clone(), name.clone()))
            .collect();
        Ok(Self {
            opcode: raw.opcode.as_str().parse()?,
            inputs,
            fields,
        })
    }
}

pub type Script = Vec<Block>;

/// Decodes scripts from a map with [`String`] and [`RawBlock`]s as key and value, respectively.
pub fn decode_scripts(blocks: &HashMap<String, RawBlock>) -> Result<Vec<Script>, DecodeError> {
    let mut scripts = Vec::new();

    for (id, raw) in blocks {
        if raw.top_level && raw.parent.is_none() {
            let mut script = Vec::new();
            let mut current_id = Some(id.clone());

            while let Some(cid) = current_id {
                let block = Block::new(&cid, blocks)?;
                current_id = blocks.get(&cid).and_then(|b| b.next.clone());
                script.push(block);
            }

            scripts.push(script);
        }
    }

    Ok(scripts)
}
