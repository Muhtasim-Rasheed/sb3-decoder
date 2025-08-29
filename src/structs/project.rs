//! A project that contains all the decoded data from a Scratch 3.0 project file (.sb3).

use std::collections::HashMap;

use zip::ZipArchive;

use crate::{
    decoder::RawProject,
    error::DecodeError,
    structs::{Broadcast, Sprite, Stage, Target, Variable},
};

/// The [`Project`] struct represents a Scratch 3.0 project.
///
/// It contains all the decoded data from a `.sb3` file, including sprites and the stage.
#[derive(Debug, Clone)]
pub struct Project {
    /// Contains all the sprites in the project.
    pub sprites: Vec<Sprite>,

    /// Contains the stage of the project.
    pub stage: Stage,
}

impl Project {
    /// Returns a new project from a [`RawProject`] and the zip archive.
    ///
    /// # Errors
    /// Returns [`DecodeError::Zip`] if there is an error reading the ZIP archive.
    /// Returns [`DecodeError::NotFound`] if something expected is not found in the archive or in
    /// the JSON data.
    pub fn new(
        raw: RawProject,
        zip: &mut ZipArchive<std::io::Cursor<Vec<u8>>>,
    ) -> Result<Self, DecodeError> {
        let mut sprites = Vec::new();
        let mut stage = None;

        for raw_target in raw.targets {
            match Target::new(raw_target, zip)? {
                Target::Sprite(sprite) => sprites.push(sprite),
                Target::Stage(s) => stage = Some(s),
            }
        }

        Ok(Self {
            sprites,
            stage: stage.ok_or(DecodeError::NotFound("Stage".to_string()))?,
        })
    }

    /// Returns a reference to the global variables from the stage.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// # let mut decoder = DecoderBuilder::new()
    /// #    .use_file("path/to/project.sb3")
    /// #    .build()
    /// #    .unwrap();
    /// let project = decoder.decode().unwrap();
    /// let global_vars = project.global_variables();
    /// ```
    pub fn global_variables(&self) -> &HashMap<String, Variable> {
        &self.stage.variables
    }

    /// Returns a mutable reference to the global variables from the stage.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// # let mut decoder = DecoderBuilder::new()
    /// #    .use_file("path/to/project.sb3")
    /// #    .build()
    /// #    .unwrap();
    /// let mut project = decoder.decode().unwrap();
    /// let mut global_vars = project.global_variables_mut();
    /// ```
    pub fn global_variables_mut(&mut self) -> &mut HashMap<String, Variable> {
        &mut self.stage.variables
    }

    /// Returns all the broadcast messages in the project.
    pub fn broadcasts(&self) -> Vec<Broadcast> {
        self.stage.broadcasts.clone()
    }

    /// Returns a reference to a sprite by its name, if it exists.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// # let mut decoder = DecoderBuilder::new()
    /// #    .use_file("path/to/project.sb3")
    /// #    .build()
    /// #    .unwrap();
    /// let project = decoder.decode().unwrap();
    /// let sprite = project.get("Cat");
    /// if let Some(sprite) = sprite {
    ///   // Do something with the sprite
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<&Sprite> {
        self.sprites.iter().find(|sprite| sprite.name == name)
    }

    /// Returns a mutable reference to a sprite by its name, if it exists.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// # let mut decoder = DecoderBuilder::new()
    /// #    .use_file("path/to/project.sb3")
    /// #    .build()
    /// #    .unwrap();
    /// let mut project = decoder.decode().unwrap();
    /// let sprite = project.get_mut("Cat");
    /// if let Some(sprite) = sprite {
    ///   // Do something with the sprite
    /// }
    /// ```
    pub fn get_mut(&mut self, name: &str) -> Option<&mut Sprite> {
        self.sprites.iter_mut().find(|sprite| sprite.name == name)
    }
}
