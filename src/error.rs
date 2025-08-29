//! Error types for the application.
//!
//! This module defines custom error types to handle various error scenarios
//! that may occur during the decoding process.

use thiserror::Error;

/// Represents errors that can occur during the decoding process.
///
/// This enum encapsulates errors from ZIP handling, JSON parsing, and I/O operations.
/// Each variant provides context about the specific error encountered.
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Error related to ZIP file handling.
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),

    /// Error related to JSON parsing.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error related to I/O operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error indicating that something was expected but not found.
    #[error("Expected item not found: {0}")]
    NotFound(String),

    /// Error indicating an invalid format or data.
    #[error("Invalid data: {0}")]
    InvalidData(String),

    /// Error indicating something was unknown or unexpected.
    #[error("Unknown {0}: {1}")]
    Unknown(String, String),
}

/// Represents errors that can occur during the building of a decoder.
///
/// This enum encapsulates errors related to invalid configurations or missing parameters.
#[derive(Error, Debug)]
pub enum BuilderError {
    /// Error indicating that no source was specified for the decoder.
    #[error("No source specified for the decoder.")]
    NoSource,

    /// Error indicating that reading the specified file failed.
    #[error("Failed to read the specified file: {0}")]
    FileRead(#[from] std::io::Error),

    /// Error that contains a DecodeError.
    #[error("Decoding error: {0}")]
    Decode(#[from] DecodeError),
}
