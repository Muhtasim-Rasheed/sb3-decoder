use std::collections::BTreeMap;

use crate::{Error, prelude::*};

/// The representation of a Scratch sprite.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Sprite {
    pub name: String,
    pub scripts: Vec<code::Script>,
    pub loose_reporters: Vec<code::Reporter>,

    pub x: f64,
    pub y: f64,
    pub direction: f64,
    pub rotation_style: RotationStyle,
    pub draggable: bool,

    pub current_costume: usize,
    pub costumes: Vec<Costume>,
    pub visible: bool,
    pub layer: u16,
    pub size: f64,

    pub sounds: Vec<Sound>,
    pub volume: f64,

    pub variables: BTreeMap<code::VarId, code::Variable>,
    pub lists: BTreeMap<code::ListId, code::List>,
}

/// The representation of a Scratch sprite's rotation style.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RotationStyle {
    #[default]
    AllAround,
    LeftRight,
    DontRotate,
}

impl std::str::FromStr for RotationStyle {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Error> {
        match value {
            "all around" => Ok(RotationStyle::AllAround),
            "left-right" => Ok(RotationStyle::LeftRight),
            "don't rotate" => Ok(RotationStyle::DontRotate),
            _ => Err(Error::InvalidRotationStyle(value.to_string())),
        }
    }
}
