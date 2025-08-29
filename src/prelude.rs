//! A prelude module to re-export commonly used items from the crate.

pub use crate::decoder::{Decoder, DecoderBuilder};
pub use crate::error::*;
pub use crate::structs::*;

#[cfg(feature = "costume_png")]
pub use image;

#[cfg(feature = "costume_svg")]
pub use usvg;
