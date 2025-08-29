//! # `sb3-decoder`
//! A Rust library for decoding Scratch 3.0 project files (.sb3) into Rust structs.
//!
//! # Example
//! ```no_run
//! use sb3_decoder::prelude::*;
//!
//! let mut decoder = DecoderBuilder::new()
//!     .use_file("path/to/project.sb3") // Open a file
//! #   .use_bytes(&[]) /*
//!     .use_bytes(include_bytes!("path/to/project.sb3")) // Or use bytes directly
//! # */
//!     .build()
//!     .unwrap();
//! let project = decoder.decode().unwrap();
//! ```
//!
//! # Features
//! - `costume_png`: Enable support for rasterizing SVG costumes to PNG.
//! - `costume_svg`: Enable support for handling SVG costumes directly.
//!
//! **Note:** The features `costume_png` and `costume_svg` are mutually exclusive; enable only one
//! of them.

#[cfg(all(feature = "costume_png", feature = "costume_svg"))]
compile_error!(
    "Features `costume_png` and `costume_svg` are mutually exclusive; enable only one of them."
);

#[cfg(all(not(feature = "costume_png"), not(feature = "costume_svg")))]
compile_error!("Either feature `costume_png` or `costume_svg` must be enabled.");

pub mod decoder;
pub mod error;
pub mod structs;
pub(crate) mod utils;

pub mod prelude;

#[cfg(feature = "costume_png")]
pub use image;

#[cfg(feature = "costume_svg")]
pub use usvg;

#[cfg(test)]
mod tests;
