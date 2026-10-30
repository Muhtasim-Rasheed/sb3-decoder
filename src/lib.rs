//! # sb3-decoder
//! A Rust crate that decodes Scratch 3.0 project files (.sb3) into rust structs and enums.
//!
//! # Example
//! ```no_run
//! use sb3_decoder::prelude::*;
//!
//! let mut decoder = {
//!     let decoder = Decoder::from_file("project.sb3"); // Read from a file
//!     # /* rust wouldn't be able to find project.sb3, so this part is commented out
//!     let decoder = Decoder::from_bytes(include_bytes!("project.sb3")); // Or load from memory
//!     # */
//!     decoder.expect("project should be valid")
//! };
//!
//! // Decode the project
//! let project = decoder.decode().expect("project should be valid");
//!
//! // Now let's print every hat and action block we find inside the Cat sprite
//! let cat = project
//!     .sprites
//!     .iter()
//!     .find(|sprite| sprite.name == "Cat")
//!     .expect("there should be a Cat sprite");
//! for script in &cat.scripts {
//!     println!("{:?}", script.hat);
//!     for action in &script.actions {
//!         println!("{action:?}");
//!     }
//! }
//! ```
//!
//! # Feature flags
//! The features `costumes` and `sounds` are enabled by default.
//!
//! - `costumes` - costume decoding support, using image and resvg.
//! - `sounds` - sound decoding support, using hound and nanomp3.

pub mod decode;
pub mod error;
pub(crate) mod macros;
pub mod model;
pub mod prelude;
pub(crate) mod raw;

pub use error::Error;

#[cfg(test)]
mod tests {
    mod decode_projects;
    mod raw_deserialization;
}
