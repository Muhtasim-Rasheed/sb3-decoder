//! This module provides all the necessary components to decode and work with Scratch 3.0 blocks.
//!
//! The main struct in this module is [`Block`], which represents a Scratch block with its
//! opcode, inputs, fields, and other properties.

pub mod block;
pub mod opcodes;
pub mod input;

pub use block::*;
pub use opcodes::*;
pub use input::*;
