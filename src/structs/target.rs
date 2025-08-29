//! The target module containing the [`Target`] enum.
//!
//! It also contains the [`Sprite`], [`Broadcast`] and [`Stage`] structs.

use std::collections::HashMap;

use indexmap::IndexMap;
use zip::ZipArchive;

use crate::{
    decoder::{RawSprite, RawStage, RawTarget},
    error::DecodeError,
    structs::{decode_scripts, Costume, List, Script, Sound, Variable},
};

/// The [`Target`] enum represents either a sprite or the stage in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub enum Target {
    /// A sprite target.
    Sprite(Sprite),

    /// The stage target.
    Stage(Stage),
}

impl Target {
    pub fn new(
        raw: RawTarget,
        zip: &mut ZipArchive<std::io::Cursor<Vec<u8>>>,
    ) -> Result<Self, DecodeError> {
        Ok(match raw {
            RawTarget::Sprite(raw_sprite) => Target::Sprite(Sprite::new(raw_sprite, zip)?),
            RawTarget::Stage(raw_stage) => Target::Stage(Stage::new(raw_stage, zip)?),
        })
    }
}

/// The [`Sprite`] struct represents a sprite in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub struct Sprite {
    /// The name of the sprite.
    pub name: String,

    /// Contains all the variables of the sprite.
    pub variables: HashMap<String, Variable>,

    /// Contains all the lists of the sprite.
    pub lists: HashMap<String, List>,

    /// Contains all the scripts of the sprite.
    pub scripts: Vec<Script>,

    /// The index of the current costume.
    pub current_costume: usize,

    /// Contains all the costumes of the sprite.
    costumes: IndexMap<String, Costume>,

    /// Contains all the sounds of the sprite.
    pub sounds: HashMap<String, Sound>,

    /// The sprite's volume (0.0 = 0%, 1.0 = 100%)
    pub volume: f32,

    /// The layer of the sprite.
    pub layer_order: isize,

    /// If the sprite is visible.
    pub visible: bool,

    /// The position of the sprite.
    pub position: (i32, i32),

    /// The size of the sprite (0.0 = 0%, 1.0 = 100%)
    pub size: f32,

    /// The direction of the sprite in degrees.
    pub direction: i32,

    /// If the sprite is draggable.
    pub draggable: bool,

    /// The rotation style of the sprite.
    pub rotation_style: RotationStyle,
}

impl Sprite {
    /// Returns a new sprite from a [`RawSprite`] and the zip archive.
    pub fn new(
        raw: RawSprite,
        zip: &mut ZipArchive<std::io::Cursor<Vec<u8>>>,
    ) -> Result<Self, DecodeError> {
        let variables = raw
            .variables
            .into_iter()
            .map(|(_, raw_var)| (raw_var.name.clone(), Variable(raw_var.value.into())))
            .collect();
        #[cfg(feature = "costume_png")]
        let costumes = raw
            .costumes
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                match raw.data_format.as_str() {
                    "svg" => {
                        let svg_data = std::str::from_utf8(&buffer).map_err(|_| {
                            DecodeError::InvalidData("SVG data is not valid UTF-8".to_string())
                        })?;
                        let svg_tree =
                            resvg::usvg::Tree::from_str(svg_data, &resvg::usvg::Options::default())
                                .map_err(|e| {
                                    DecodeError::InvalidData(format!("Failed to parse SVG: {}", e))
                                })?;
                        let image = crate::utils::rasterize_svg_to_png(
                            &svg_tree,
                            svg_tree.size().width() as u32,
                            svg_tree.size().height() as u32,
                        )
                        .map_err(|e| {
                            DecodeError::InvalidData(format!("Failed to rasterize SVG: {}", e))
                        })?;
                        Ok((raw.name, Costume(image)))
                    }
                    _ => {
                        let image = image::load_from_memory(&buffer).map_err(|e| {
                            DecodeError::InvalidData(format!("Failed to load image: {}", e))
                        })?;
                        Ok((raw.name, Costume(image)))
                    }
                }
            })
            .collect::<Result<IndexMap<String, Costume>, DecodeError>>()?;
        #[cfg(feature = "costume_svg")]
        let costumes = raw
            .costumes
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                match raw.data_format.as_str() {
                    "svg" => {
                        let svg_data = std::str::from_utf8(&buffer).map_err(|_| {
                            DecodeError::InvalidData("SVG data is not valid UTF-8".to_string())
                        })?;
                        let svg_tree = usvg::Tree::from_str(svg_data, &usvg::Options::default())
                            .map_err(|e| {
                                DecodeError::InvalidData(format!("Failed to parse SVG: {}", e))
                            })?;
                        Ok((raw.name, Costume(svg_tree)))
                    }
                    _ => Err(DecodeError::InvalidData(format!(
                        "Unsupported data format: {}",
                        raw.data_format
                    ))),
                }
            })
            .collect::<Result<IndexMap<String, Costume>, DecodeError>>()?;
        let sounds = raw
            .sounds
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                let wav_reader = hound::WavReader::new(&buffer[..]).map_err(|e| {
                    DecodeError::InvalidData(format!("Failed to read WAV data: {}", e))
                })?;
                let spec = wav_reader.spec();
                let samples: Vec<i16> = wav_reader
                    .into_samples::<i16>()
                    .collect::<Result<_, _>>()
                    .map_err(|e| {
                        DecodeError::InvalidData(format!("Failed to read WAV samples: {}", e))
                    })?;
                Ok((
                    raw.name.clone(),
                    Sound {
                        data: samples,
                        rate: spec.sample_rate,
                    },
                ))
            })
            .collect::<Result<HashMap<String, Sound>, DecodeError>>()?;
        let lists = raw
            .lists
            .into_iter()
            .map(|(_, raw_list)| {
                (
                    raw_list.0.clone(),
                    List(raw_list.1.iter().map(|v| v.clone().into()).collect()),
                )
            })
            .collect();

        Ok(Self {
            name: raw.name,
            variables,
            lists,
            scripts: decode_scripts(&raw.blocks)?,
            current_costume: raw.current_costume,
            costumes,
            sounds,
            volume: raw.volume as f32 / 100.0,
            layer_order: raw.layer_order,
            visible: raw.visible,
            position: (raw.x, raw.y),
            size: raw.size as f32 / 100.0,
            direction: raw.direction,
            draggable: raw.draggable,
            rotation_style: RotationStyle::try_from(raw.rotation_style.as_str())
                .map_err(|_| DecodeError::InvalidData("Invalid rotation style".to_string()))?,
        })
    }

    /// Returns a reference to a costume by its name, if it exists.
    pub fn get_costume(&self, name: &str) -> Option<&Costume> {
        self.costumes.get(name)
    }

    /// Returns a mutable reference to a costume by its name, if it exists.
    pub fn get_costume_mut(&mut self, name: &str) -> Option<&mut Costume> {
        self.costumes.get_mut(name)
    }

    /// Returns all the costume names in the sprite.
    pub fn costume_names(&self) -> Vec<&String> {
        self.costumes.keys().collect()
    }

    /// Returns all the costumes in the sprite.
    pub fn costumes(&self) -> Vec<&Costume> {
        self.costumes.values().collect()
    }
}

/// The [`Stage`] struct represents the stage in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub struct Stage {
    /// Contains all the global variables in the project.
    pub variables: HashMap<String, Variable>,

    // Contains all the global lists in the project.
    pub lists: HashMap<String, List>,

    /// Contains all the broadcasts in the project.
    pub broadcasts: Vec<Broadcast>,

    /// Contains all the scripts in the stage.
    pub scripts: Vec<Script>,

    /// The index of the current backdrop.
    pub current_backdrop: usize,

    /// Contains all the backdrops in the project.
    backdrops: IndexMap<String, Costume>,

    /// Contains all the sounds of the stage.
    pub sounds: HashMap<String, Sound>,

    /// The stage's volume (0.0 = 0%, 1.0 = 100%)
    pub volume: f32,
}

impl Stage {
    /// Returns a new stage from a [`RawStage`] and the zip archive.
    pub fn new(
        raw: RawStage,
        zip: &mut ZipArchive<std::io::Cursor<Vec<u8>>>,
    ) -> Result<Self, DecodeError> {
        let variables = raw
            .variables
            .into_iter()
            .map(|(_, raw_var)| (raw_var.name.clone(), Variable(raw_var.value.into())))
            .collect();
        #[cfg(feature = "costume_png")]
        let backdrops = raw
            .backdrops
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                match raw.data_format.as_str() {
                    "svg" => {
                        let svg_data = std::str::from_utf8(&buffer).map_err(|_| {
                            DecodeError::InvalidData("SVG data is not valid UTF-8".to_string())
                        })?;
                        let svg_tree =
                            resvg::usvg::Tree::from_str(svg_data, &resvg::usvg::Options::default())
                                .map_err(|e| {
                                    DecodeError::InvalidData(format!("Failed to parse SVG: {}", e))
                                })?;
                        let image = crate::utils::rasterize_svg_to_png(
                            &svg_tree,
                            svg_tree.size().width() as u32,
                            svg_tree.size().height() as u32,
                        )
                        .map_err(|e| {
                            DecodeError::InvalidData(format!("Failed to rasterize SVG: {}", e))
                        })?;
                        Ok((raw.name, Costume(image)))
                    }
                    _ => {
                        let image = image::load_from_memory(&buffer).map_err(|e| {
                            DecodeError::InvalidData(format!("Failed to load image: {}", e))
                        })?;
                        Ok((raw.name, Costume(image)))
                    }
                }
            })
            .collect::<Result<IndexMap<String, Costume>, DecodeError>>()?;
        #[cfg(feature = "costume_svg")]
        let backdrops = raw
            .backdrops
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                match raw.data_format.as_str() {
                    "svg" => {
                        let svg_data = std::str::from_utf8(&buffer).map_err(|_| {
                            DecodeError::InvalidData("SVG data is not valid UTF-8".to_string())
                        })?;
                        let svg_tree = usvg::Tree::from_str(svg_data, &usvg::Options::default())
                            .map_err(|e| {
                                DecodeError::InvalidData(format!("Failed to parse SVG: {}", e))
                            })?;
                        Ok((raw.name, Costume(svg_tree)))
                    }
                    _ => Err(DecodeError::InvalidData(format!(
                        "Unsupported data format: {}",
                        raw.data_format
                    ))),
                }
            })
            .collect::<Result<IndexMap<String, Costume>, DecodeError>>()?;
        let sounds = raw
            .sounds
            .into_iter()
            .map(|raw| {
                let mut file = zip.by_name(&raw.md5ext)
                    .map_err(|_| DecodeError::NotFound(raw.md5ext.clone()))?;
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                let wav_reader = hound::WavReader::new(&buffer[..]).map_err(|e| {
                    DecodeError::InvalidData(format!("Failed to read WAV data: {}", e))
                })?;
                let spec = wav_reader.spec();
                let samples: Vec<i16> = wav_reader
                    .into_samples::<i16>()
                    .collect::<Result<_, _>>()
                    .map_err(|e| {
                        DecodeError::InvalidData(format!("Failed to read WAV samples: {}", e))
                    })?;
                Ok((
                    raw.name.clone(),
                    Sound {
                        data: samples,
                        rate: spec.sample_rate,
                    },
                ))
            })
            .collect::<Result<HashMap<String, Sound>, DecodeError>>()?;
        let lists = raw
            .lists
            .into_iter()
            .map(|(_, raw_list)| {
                (
                    raw_list.0.clone(),
                    List(raw_list.1.iter().map(|v| v.clone().into()).collect()),
                )
            })
            .collect();

        Ok(Self {
            variables,
            lists,
            broadcasts: raw
                .broadcasts
                .into_iter()
                .map(|(_, msg)| Broadcast(msg))
                .collect(),
            scripts: decode_scripts(&raw.blocks)?,
            current_backdrop: raw.current_backdrop,
            backdrops,
            sounds,
            volume: raw.volume as f32 / 100.0,
        })
    }

    /// Returns a reference to a backdrop by its name, if it exists.
    pub fn get_backdrop(&self, name: &str) -> Option<&Costume> {
        self.backdrops.get(name)
    }

    /// Returns a mutable reference to a backdrop by its name, if it exists.
    pub fn get_backdrop_mut(&mut self, name: &str) -> Option<&mut Costume> {
        self.backdrops.get_mut(name)
    }

    /// Returns all the backdrop names in the stage.
    pub fn backdrop_names(&self) -> Vec<&String> {
        self.backdrops.keys().collect()
    }

    /// Returns all the backdrops in the stage.
    pub fn backdrops(&self) -> Vec<&Costume> {
        self.backdrops.values().collect()
    }
}

/// The [`Broadcast`] struct represents a usable broadcast message in a Scratch 3.0 project.
#[derive(Debug, Clone)]
pub struct Broadcast(pub String);

/// The [`RotationStyle`] enum represents the rotation style of a sprite in a Scratch 3.0 project.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationStyle {
    /// The sprite rotates freely.
    AllAround,

    /// The sprite rotates left and right.
    LeftRight,

    /// The sprite does not rotate.
    DontRotate,
}

impl TryFrom<&str> for RotationStyle {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all around" => Ok(RotationStyle::AllAround),
            "left-right" => Ok(RotationStyle::LeftRight),
            "don't rotate" => Ok(RotationStyle::DontRotate),
            _ => Err("Invalid rotation style"),
        }
    }
}
