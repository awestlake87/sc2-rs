use sc2_proto::common;

use super::super::{FromProto, Result};

/// data used to sample the current state of the map at certain points
#[derive(Debug, Clone)]
pub struct ImageData {
    bits_per_pixel: u32,
    data: Vec<u8>,
    dimensions: (u32, u32),
}

impl ImageData {
    /// number of bits to interpret as a pixel in the data
    pub fn get_bpp(&self) -> u32 {
        self.bits_per_pixel
    }

    /// raw image data
    pub fn get_raw_data(&self) -> &[u8] {
        &self.data
    }

    /// dimensions of the image
    pub fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }
}

impl FromProto<common::ImageData> for ImageData {
    fn from_proto(mut data: common::ImageData) -> Result<Self> {
        Ok(Self {
            bits_per_pixel: data.get_bits_per_pixel() as u32,
            data: data.take_data(),
            dimensions: (
                data.get_size().get_x() as u32,
                data.get_size().get_y() as u32,
            ),
        })
    }
}
