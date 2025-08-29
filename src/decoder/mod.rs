//! The decoder for decoding Scratch 3.0 project files (.sb3).
//!
//! This module provides functionality to decode .sb3 files into Rust structs.
//! It includes the [`Decoder`] struct and related methods for handling the decoding process.

pub mod decoder;
pub mod decoder_builder;
pub mod raw_structs;

pub use decoder::*;
pub use decoder_builder::*;
pub use raw_structs::*;
