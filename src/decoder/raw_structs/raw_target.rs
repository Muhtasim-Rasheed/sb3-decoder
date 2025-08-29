//! The raw_target module contains the [`RawTarget`] enum.
//!
//! It also contains the [`RawSprite`], [`RawVariable`] and [`RawStage`] structs.

use std::collections::HashMap;

use crate::decoder::{RawBlock, RawCostume, RawSound};

/// The [`RawTarget`] enum represents either a sprite or the stage in its raw form in a Scratch 3.0
/// project.
pub enum RawTarget {
    /// A sprite target.
    Sprite(RawSprite),

    /// The stage target.
    Stage(RawStage),
}

impl<'de> serde::Deserialize<'de> for RawTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;
        let is_stage = v
            .get("isStage")
            .and_then(|val| val.as_bool())
            .ok_or_else(|| serde::de::Error::missing_field("isStage"))?;
        if is_stage {
            let stage: RawStage = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(RawTarget::Stage(stage))
        } else {
            let sprite: RawSprite = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
            Ok(RawTarget::Sprite(sprite))
        }
    }
}

/// The [`RawSprite`] struct represents a sprite in its raw form in a Scratch 3.0 project.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSprite {
    /// The name of the sprite.
    pub name: String,

    /// The variables of the sprite.
    pub variables: HashMap<String, RawVariable>,

    /// The lists of the sprite.
    pub lists: HashMap<String, (String, Vec<serde_json::Value>)>,

    /// The blocks of the sprite stored as a map from block ID to [`RawBlock`].
    pub blocks: HashMap<String, RawBlock>,

    /// The index of the current costume.
    pub current_costume: usize,

    /// The costumes of the sprite.
    pub costumes: Vec<RawCostume>,

    /// The sounds of the sprite.
    pub sounds: Vec<RawSound>,

    /// The volume of the sprite.
    pub volume: u8,

    /// The layer of the sprite.
    pub layer_order: isize,

    /// If the sprite is visible.
    pub visible: bool,

    /// The x position of the sprite.
    pub x: i32,

    /// The y position of the sprite.
    pub y: i32,

    /// The size of the sprite.
    pub size: u32,

    /// The direction of the sprite.
    pub direction: i32,

    /// If the sprite is draggable.
    pub draggable: bool,

    /// The rotation style of the sprite.
    pub rotation_style: String,
}

/// The [`RawStage`] struct represents the stage in its raw form in a Scratch 3.0 project.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawStage {
    /// All the global variables that can be accessed by all sprites.
    pub variables: HashMap<String, RawVariable>,

    /// All the global lists that can be accessed by all sprites.
    pub lists: HashMap<String, (String, Vec<serde_json::Value>)>,

    /// All the broadcasts that can be used by all sprites.
    pub broadcasts: HashMap<String, String>,

    /// The blocks of the stage stored as a map from block ID to [`RawBlock`].
    pub blocks: HashMap<String, RawBlock>,

    /// The index of the current backdrop.
    #[serde(rename = "currentCostume")]
    pub current_backdrop: usize,

    /// The backdrops of the stage.
    #[serde(rename = "costumes")]
    pub backdrops: Vec<RawCostume>,

    /// The sounds of the stage.
    pub sounds: Vec<RawSound>,

    /// The volume of the stage.
    pub volume: u8,
}

/// The [`RawVariable`] struct represents a variable in its raw form in a Scratch 3.0 project.
#[derive(serde::Deserialize)]
pub struct RawVariable {
    /// The name of the variable.
    pub name: String,

    /// The value of the variable.
    pub value: serde_json::Value,
}
