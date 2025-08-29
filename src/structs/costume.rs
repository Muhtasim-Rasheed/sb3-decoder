//! The costume module contains the [`Costume`] struct and related functionality.

/// The [`Costume`] struct represents a costume / backdrop in a Scratch 3.0 project.
#[cfg(feature = "costume_png")]
#[derive(Debug, Clone)]
pub struct Costume(
    /// The image data of the costume.
    pub image::DynamicImage,
);

#[cfg(feature = "costume_svg")]
#[derive(Debug, Clone)]
pub struct Costume(
    /// The SVG tree of the costume.
    pub usvg::Tree,
);
