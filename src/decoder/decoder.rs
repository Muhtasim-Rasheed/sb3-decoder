//! A decoder module for handling Scratch 3.0 project files (.sb3).

use crate::{decoder::RawProject, error::DecodeError, structs::Project};

/// A decoder for Scratch 3.0 project files (.sb3).
///
/// The [`Decoder`] struct provides functionality to decode `.sb3` files into Rust structs.
pub struct Decoder {
    archive: zip::ZipArchive<std::io::Cursor<Vec<u8>>>,
}

impl Decoder {
    /// Creates a new [`Decoder`] from the provided bytes of a `.sb3` file.
    ///
    /// # Errors
    /// Returns [`DecodeError::Zip`] if there is an error reading the ZIP archive.
    pub(super) fn new(bytes: Vec<u8>) -> Result<Self, DecodeError> {
        let cursor = std::io::Cursor::new(bytes);
        let archive = zip::ZipArchive::new(cursor)?;
        Ok(Self { archive })
    }

    /// Decodes the `.sb3` file and returns the project data as a [`RawProject`] struct.
    ///
    /// # Errors
    /// Returns [`DecodeError::Json`] if there is an error parsing the JSON data.
    pub fn decode_raw(&mut self) -> Result<RawProject, DecodeError> {
        let project_file = self.archive.by_name("project.json")?;
        let raw_project: RawProject = serde_json::from_reader(project_file)?;
        Ok(raw_project)
    }

    /// Decodes the `.sb3` file and returns the project data as a [`Project`] struct.
    ///
    /// # Errors
    /// Returns [`DecodeError::Json`] if there is an error parsing the JSON data.  
    /// Returns [`DecodeError::NotFound`] if something expected is not found in the archive or in
    /// the JSON data.
    pub fn decode(&mut self) -> Result<Project, DecodeError> {
        let raw_project = self.decode_raw()?;
        let project = Project::new(raw_project, &mut self.archive)?;
        Ok(project)
    }
}
