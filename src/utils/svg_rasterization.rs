//! The svg_rasterization module provides functionality to rasterize SVG images into the PNG format.

use image::DynamicImage;
use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::Tree;

fn pixmap_to_dynamic_image(pixmap: &Pixmap) -> DynamicImage {
    let mut buf = Vec::with_capacity(pixmap.data().len());
    for chunk in pixmap.data().chunks_exact(4) {
        let (r, g, b, a) = (chunk[0], chunk[1], chunk[2], chunk[3]);
        if a > 0 {
            let alpha = a as f32 / 255.0;
            buf.push(((r as f32 / alpha).min(255.0)) as u8);
            buf.push(((g as f32 / alpha).min(255.0)) as u8);
            buf.push(((b as f32 / alpha).min(255.0)) as u8);
            buf.push(a);
        } else {
            buf.extend_from_slice(&[0, 0, 0, 0]);
        }
    }
    let img_buffer = image::RgbaImage::from_raw(pixmap.width(), pixmap.height(), buf)
        .expect("Failed to create image buffer");
    DynamicImage::ImageRgba8(img_buffer)
}

pub fn rasterize_svg_to_png(
    svg_tree: &Tree,
    width: u32,
    height: u32,
) -> Result<DynamicImage, String> {
    let mut pixmap =
        Pixmap::new(width, height).ok_or_else(|| "Failed to create pixmap".to_string())?;
    resvg::render(&svg_tree, Transform::default(), &mut pixmap.as_mut());
    Ok(pixmap_to_dynamic_image(&pixmap))
}
