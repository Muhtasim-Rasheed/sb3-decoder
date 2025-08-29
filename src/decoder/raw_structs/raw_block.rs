//! The raw_block module contains the [`RawBlock`] struct and related functionality.

use std::collections::HashMap;

/// The [`RawBlock`] struct represents a block in its raw form in a Scratch 3.0 project.
/// It contains the opcode, inputs, fields, next block, parent block, and shadow status.
///
/// Please refer to the [Scratch
/// Wiki](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Blocks) for more details on the
/// structure of blocks in Scratch projects.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawBlock {
    /// The opcode of the block (e.g., "event_whenflagclicked", "motion_movesteps").
    pub opcode: String,

    /// The next block ID in the sequence. `None` if there is no next block and the script ends
    /// here.
    pub next: Option<String>,

    /// The previous block ID in the sequence if the block is a top-level block. If not, and this
    /// block is a child of another block, this will link to the caller block.
    pub parent: Option<String>,

    /// The inputs to the block, represented as a map from input name to a [`RawBlockInput`].
    pub inputs: HashMap<String, RawBlockInput>,

    /// The fields of the block, represented as a map from field name to a tuple containing a
    /// variable name and its ID.
    pub fields: HashMap<String, (String, Option<String>)>,

    /// Indicates whether the block is a top-level block in a script (cap block).
    pub top_level: bool,
}

/// The [`RawBlockInput`] struct represents an input to a block in its raw form in a Scratch 3.0
/// project.
#[derive(Debug, Clone, PartialEq)]
pub struct RawBlockInput {
    /// The shadow type of the block.  
    /// 1 for shadow block,  
    /// 2 for non-shadow block.
    /// 3 for shadow block but is obscured by a non-shadow block.
    pub shadow: u8,

    /// The type of the input.  
    /// 4 to 8 for a number.  
    /// 9 for hex color string.  
    /// 10 for string.  
    /// 11 for broadcast.  
    /// 12 and 13 for variables and lists, respectively.
    ///
    /// A variant is used by this crate to represent a block reference (i.e., when the input is
    /// another block). In this case, the `kind` will be `0`, and the `value` will be a string
    /// of the block ID.
    pub kind: u8,

    /// The value of the input.  
    /// number for 4 to 8,  
    /// string for 9 to 13.
    pub value: serde_json::Value,
}

impl<'de> serde::Deserialize<'de> for RawBlockInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let vec: Vec<serde_json::Value> = serde::Deserialize::deserialize(deserializer)?;

        if vec.len() < 2 {
            return Err(serde::de::Error::custom("Invalid RawBlockInput format"));
        }

        let shadow = vec[0]
            .as_u64()
            .ok_or_else(|| serde::de::Error::custom("Invalid shadow value"))? as u8;

        // Case 1: index 1 is a block ID (string)
        if let Some(block_id) = vec[1].as_str() {
            return Ok(RawBlockInput {
                shadow,
                kind: 0, // Use 0 or a special marker for "block ref"
                value: serde_json::Value::String(block_id.to_string()),
            });
        }

        // Case 2: index 1 is an array [kind, value]
        if let Some(sub_vec) = vec[1].as_array() {
            if sub_vec.len() < 2 {
                return Err(serde::de::Error::custom("Invalid sub-array format"));
            }

            let kind = sub_vec[0]
                .as_u64()
                .ok_or_else(|| serde::de::Error::custom("Invalid type value"))? as u8;

            let value = sub_vec[1].clone();

            return Ok(RawBlockInput { shadow, kind, value });
        }

        Err(serde::de::Error::custom("Invalid RawBlockInput second element"))
    }
}
