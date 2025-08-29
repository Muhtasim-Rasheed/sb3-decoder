//! The raw_project module contains the [`RawProject`] struct and related functionality.

use crate::decoder::RawTarget;

/// The [`RawProject`] struct represents a Scratch 3.0 project json file.
#[derive(serde::Deserialize)]
pub struct RawProject {
    /// Contains all the targets (sprites and stage) in the project.
    pub targets: Vec<RawTarget>,
}
