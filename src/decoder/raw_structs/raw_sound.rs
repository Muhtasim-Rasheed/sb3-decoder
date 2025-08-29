//! The raw_sound module contains the [`RawSound`] struct.

/// The [`RawSound`] struct represents a sound in its raw form in a
/// Scratch 3.0 project.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSound {
    /// The name of the sound.
    pub name: String,

    /// The MD5 hash with file extension of the sound.
    pub md5ext: String,
}
