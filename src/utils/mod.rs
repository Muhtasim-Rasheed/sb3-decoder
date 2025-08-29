//! The utils module that provides various utility functions that
//! are used internally by the crate.

#[cfg(feature = "costume_png")]
pub mod svg_rasterization;

#[cfg(feature = "costume_png")]
pub use svg_rasterization::*;
