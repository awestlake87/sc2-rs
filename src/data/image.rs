
use sc2_proto::common;

use super::super::{ Result, FromProto };

/// data used to sample the current state of the map at certain points
#[derive(Debug, Clone)]
pub struct ImageData {
    /// number of bits to interpret as a pixel in the data
    pub bits_per_pixel:         i32,
    /// raw image data
    pub data:                   Vec<u8>,

    /// width of the image
    pub width:                  i32,
    /// height of the image
    pub height:                 i32,
}

impl FromProto<common::ImageData> for ImageData {
    fn from_proto(mut data: common::ImageData) -> Result<Self> {
        Ok(
            Self {
                bits_per_pixel: data.get_bits_per_pixel(),
                data: data.take_data(),
                width: data.get_size().get_x(),
                height: data.get_size().get_y()
            }
        )
    }
}
