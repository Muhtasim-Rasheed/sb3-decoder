use crate::prelude::*;

/// The representation of a Scratch stage.
///
/// In the project.json file, all the global variables and lists are stored inside the stage. They
/// are moved to the [`Project`] struct when decoding and not here.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Stage {
    pub scripts: Vec<code::Script>,
    pub loose_reporters: Vec<code::Reporter>,

    pub current_backdrop: usize,
    pub backdrops: Vec<Costume>,

    pub sounds: Vec<Sound>,
    pub volume: f64,
}
