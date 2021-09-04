use std::usize;

use crate::types::Frame;

pub mod color_map;
pub mod extension_block;
pub mod image_descriptor;
pub mod screen_descriptor;
pub mod signature;

pub fn encode_frames(
    bytes: &mut [u8],
    frames: &[Frame],
    cursor: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut cursor = cursor;

    for frame in frames {
        cursor = encode_frame(bytes, frame, cursor)?;
    }

    Ok(cursor)
}

pub fn encode_frame(
    bytes: &mut [u8],
    frame: &Frame,
    cursor: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    // todo encode image descriptor
    // todo encode local color map
    // todo encode raster data (rgba raster data or raster data + local/global color map)
    let mut cursor = cursor;
    if let Some(graphic_control_extension_block) = frame.graphic_control_extension() {
        cursor = extension_block::encode_graphic_control_extension(
            bytes,
            graphic_control_extension_block,
            cursor,
        );
    }

    Ok(cursor)
}

#[test]
pub fn should_encode() {}
