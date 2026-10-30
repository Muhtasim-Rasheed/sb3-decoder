#![allow(dead_code)]

use std::collections::BTreeMap;
// we use btreemap because hashmaps and snapshot testing dont go well together

use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use serde_json::Value as Json;

#[derive(Debug, Deserialize)]
pub struct Project {
    pub targets: Vec<Target>,
}

#[serde_inline_default]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    pub is_stage: bool,
    pub name: String,
    #[serde(default)]
    pub variables: BTreeMap<String, Variable>,
    #[serde(default)]
    pub lists: BTreeMap<String, List>,
    #[serde(default)]
    pub broadcasts: BTreeMap<String, String>,
    pub blocks: BTreeMap<String, Block>,
    pub current_costume: usize,
    pub costumes: Vec<Costume>,
    pub sounds: Vec<Sound>,
    pub volume: f64,
    pub layer_order: u16,

    // only appears when target is a sprite
    #[serde_inline_default(true)]
    pub visible: bool,
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde_inline_default(100.0)]
    pub size: f64,
    #[serde_inline_default(90.0)]
    pub direction: f64,
    #[serde(default)]
    pub draggable: bool,
    pub rotation_style: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Variable(pub String, pub Json, #[serde(default)] pub bool);

#[derive(Debug, Deserialize)]
pub struct List(pub String, pub Vec<Json>);

#[derive(Debug)]
pub enum Block {
    Full(FullBlock),
    // top-level variable/list reporters are stored as bare arrays: [12, "name", "id", x?, y?]
    Primitive(Vec<Json>),
}

// manual implementation for better error messages
impl<'de> Deserialize<'de> for Block {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{
            MapAccess, SeqAccess, Visitor,
            value::{MapAccessDeserializer, SeqAccessDeserializer},
        };

        struct BlockVisitor;

        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Block;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a block object or a primitive array")
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Block, A::Error> {
                FullBlock::deserialize(MapAccessDeserializer::new(map)).map(Block::Full)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, seq: A) -> Result<Block, A::Error> {
                Vec::<Json>::deserialize(SeqAccessDeserializer::new(seq)).map(Block::Primitive)
            }
        }

        deserializer.deserialize_any(BlockVisitor)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullBlock {
    pub opcode: String,
    pub next: Option<String>,
    pub parent: Option<String>,
    #[serde(default)]
    pub inputs: BTreeMap<String, Input>,
    #[serde(default)]
    pub fields: BTreeMap<String, Field>,
    #[serde(default)]
    pub shadow: bool,
    #[serde(default)]
    pub top_level: bool,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub mutation: Option<Mutation>,
}

#[derive(Debug, Deserialize)]
pub struct Input(
    pub u8,
    pub InputValue,
    #[serde(default)] pub Option<InputValue>,
);

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum InputValue {
    Block(String),
    // primitive might be invalid, we check this while decoding from raw to model
    Primitive(Vec<Json>),
    Empty,
}

#[derive(Debug, Deserialize)]
pub struct Field(pub String, pub Option<String>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mutation {
    pub tag_name: String,
    #[serde(default)]
    pub children: Vec<Json>,

    // only appears in `procedures_prototype` and `procedures_call`
    pub proccode: Option<String>,
    pub argumentids: Option<String>,
    pub argumentnames: Option<String>,
    pub argumentdefaults: Option<String>,
    pub warp: Option<String>,

    // only appears in `control_stop`
    pub hasnext: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Costume {
    pub name: String,
    pub bitmap_resolution: Option<f64>,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sound {
    pub name: String,
    pub data_format: String,
    pub asset_id: String,
    pub md5ext: String,
    pub rate: u32,
    pub sample_count: u32,
}
