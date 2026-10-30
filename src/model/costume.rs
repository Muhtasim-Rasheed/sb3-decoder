use crate::Error;

/// The representation of a Scratch costume inside a sprite.
#[derive(Clone)]
#[non_exhaustive]
pub struct Costume {
    pub name: String,
    pub data_format: CostumeFormat,
    pub rotation_center_x: f64,
    pub rotation_center_y: f64,
    pub bitmap_resolution: f64,

    /// Raw file data. Enable the `costumes` feature to decode it.
    pub data: Vec<u8>,
}

impl std::fmt::Debug for Costume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            data_format,
            rotation_center_x,
            rotation_center_y,
            bitmap_resolution,
            data,
        } = self;

        f.debug_struct("Costume")
            .field("name", name)
            .field("data_format", data_format)
            .field("rotation_center_x", rotation_center_x)
            .field("rotation_center_y", rotation_center_y)
            .field("bitmap_resolution", bitmap_resolution)
            .field("data_len", &data.len())
            .finish()
    }
}

/// The image format the costume image is stored in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CostumeFormat {
    Png,
    Jpeg,
    Bmp,
    Svg,
}

#[cfg(feature = "costumes")]
impl Costume {
    /// Decodes the image data inside this costume.
    pub fn decode(&self) -> Result<DecodedCostume, Error> {
        match self.data_format {
            CostumeFormat::Png => self.decode_bitmap(image::ImageFormat::Png),
            CostumeFormat::Jpeg => self.decode_bitmap(image::ImageFormat::Jpeg),
            CostumeFormat::Bmp => self.decode_bitmap(image::ImageFormat::Bmp),
            CostumeFormat::Svg => self.decode_svg(),
        }
    }

    fn decode_bitmap(&self, format: image::ImageFormat) -> Result<DecodedCostume, Error> {
        let image =
            image::load_from_memory_with_format(&self.data, format).map_err(Error::image)?;
        Ok(DecodedCostume {
            width: image.width(),
            height: image.height(),
            rgba: image.into_rgba8().into_vec(),
        })
    }

    fn decode_svg(&self) -> Result<DecodedCostume, Error> {
        use resvg::tiny_skia::{Pixmap, Transform};
        use resvg::usvg::{Options, Tree};

        let mut opt = Options::default();
        opt.fontdb_mut().load_system_fonts();

        let tree = Tree::from_data(&self.data, &opt).map_err(Error::usvg)?;
        let size = tree.size();
        let (w, h) = (size.width().ceil() as u32, size.height().ceil() as u32);

        let mut pixmap = Pixmap::new(w, h).ok_or_else(|| Error::InvalidCostumeSize { w, h })?;
        resvg::render(&tree, Transform::default(), &mut pixmap.as_mut());

        let rgba = pixmap
            .pixels()
            .iter()
            .flat_map(|p| {
                let c = p.demultiply();
                [c.red(), c.green(), c.blue(), c.alpha()]
            })
            .collect();

        Ok(DecodedCostume {
            width: w,
            height: h,
            rgba,
        })
    }
}

/// A decoded costume file.
#[cfg(feature = "costumes")]
#[derive(Debug, Clone)]
pub struct DecodedCostume {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}
