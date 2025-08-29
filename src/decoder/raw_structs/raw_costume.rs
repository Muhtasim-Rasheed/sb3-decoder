//! The raw_costume module contains the [`RawCostume`] struct.

/// The [`RawCostume`] struct represents a costume / backdrop in its raw form
/// in a Scratch 3.0 project.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCostume {
    /// The name of the costume / backdrop.
    pub name: String,

    /// The data format of the costume (e.g., "png", "svg").
    pub data_format: String,

    /// The MD5 hash with file extension of the costume / backdrop.
    pub md5ext: String,
}
