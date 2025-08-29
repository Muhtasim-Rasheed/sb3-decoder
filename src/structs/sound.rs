//! The sound module contains the [`Sound`] struct and related functionality.

/// The [`Sound`] struct represents a sound in a Scratch 3.0 project.
///
/// Fun fact: The spec data in the JSON file is often incorrect but the
/// wave file itself is fine. Don't trust the spec data from the JSON!
#[derive(Debug, Clone)]
pub struct Sound {
    /// The PCM sample data of the sound.
    pub data: Vec<i16>,

    /// The sample rate of the sound.
    pub rate: u32,
}
