//! A builder for creating a Decoder from either a file or in-memory bytes.
//!
//! # Example
//! ```no_run
//! # use sb3_decoder::prelude::*;
//! let builder = DecoderBuilder::new()
//!     .use_file("path/to/project.sb3");
//!
//! let decoder = match builder.build() {
//!     Ok(decoder) => decoder,
//!     Err(e) => panic!("Failed to build decoder: {}", e),
//! };
//! ```

use std::path::PathBuf;

use crate::{decoder::Decoder, error::BuilderError};

/// Represents the source of the .sb3 data.
enum Source {
    /// No source specified.
    None,

    /// Source from a file path.
    File(PathBuf),

    /// Source from in-memory bytes.
    Memory(Vec<u8>),
}

/// A builder for creating a [`Decoder`] from either a file or in-memory bytes.
///
/// # Example
/// ```no_run
/// # use sb3_decoder::prelude::*;
/// let builder = DecoderBuilder::new()
///     .use_file("path/to/project.sb3");
///
/// let decoder = match builder.build() {
///     Ok(decoder) => decoder,
///     Err(e) => panic!("Failed to build decoder: {}", e),
/// };
/// ```
pub struct DecoderBuilder {
    source: Source,
}

impl Default for DecoderBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DecoderBuilder {
    /// Creates a new [`DecoderBuilder`] instance with no source specified.
    pub fn new() -> Self {
        Self {
            source: Source::None,
        }
    }

    /// Specifies the source as a file path.
    ///
    /// # Example
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// let builder = DecoderBuilder::new()
    ///     .use_file("path/to/project.sb3");
    /// ```
    pub fn use_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.source = Source::File(path.into());
        self
    }

    /// Specifies the source as in-memory bytes.
    ///
    /// # Example
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// let builder = DecoderBuilder::new()
    /// #   .use_bytes(&[]); /*
    ///     .use_bytes(include_bytes!("path/to/project.sb3"));
    /// # */
    /// ```
    pub fn use_bytes(mut self, bytes: &[u8]) -> Self {
        self.source = Source::Memory(bytes.to_vec());
        self
    }

    /// Builds the [`Decoder`] from the specified source.
    ///
    /// # Example
    /// ```no_run
    /// # use sb3_decoder::prelude::*;
    /// # let builder = DecoderBuilder::new()
    /// #     .use_file("path/to/project.sb3");
    /// let decoder = match builder.build() {
    ///   Ok(decoder) => decoder,
    ///   Err(e) => panic!("Failed to build decoder: {}", e),
    /// };
    /// ```
    ///
    /// # Errors
    /// Returns [`BuilderError::FileRead`] if there is an error reading the file.  
    /// Returns [`BuilderError::NoSource`] if no source was specified.  
    /// Returns [`BuilderError::Decode`] if there is an error creating the decoder.
    pub fn build(self) -> Result<Decoder, BuilderError> {
        let bytes = match self.source {
            Source::File(path) => std::fs::read(path)?,
            Source::Memory(data) => data,
            Source::None => return Err(BuilderError::NoSource),
        };

        Ok(Decoder::new(bytes)?)
    }
}
