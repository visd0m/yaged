#[cfg(test)]
use std::fs::File;
use std::io::Read;
#[cfg(test)]
use std::path::Path;
use crate::types::{Gif, Frame, ColorMap, ExtensionBlock, GraphicControlExtension};

pub mod steps;

/// Color output mode
#[derive(PartialEq)]
pub enum ColorOutput {
    /// Every byte of the raster data is expanded to 4 bytes (R G B A).
    /// In this mode the ColorMap is useless, for this reason it is not returned in the Gif object.
    RGBA,
    /// Normal ColorMap index color mapping.
    ColorMap,
}

/// Decode a gif encoded source.
pub fn decode(
    mut source: impl Read,
    color_output: ColorOutput,
) -> Result<Gif, Box<dyn std::error::Error>> {
    let bytes: &mut Vec<u8> = &mut Vec::new();
    source.read_to_end(bytes)?;

    let (signature, cursor) = steps::signature::decode(&bytes, 0);
    let (mut screen_descriptor, cursor) = steps::screen_descriptor::decode(&bytes, cursor);
    let (mut global_color_map, cursor) =
        steps::color_map::decode(&bytes, screen_descriptor.pixel, screen_descriptor.m, cursor);
    let (mut frames, _cursor) = frames(bytes, cursor);

    if color_output == ColorOutput::RGBA {
        for frame in &mut frames {
            let rgba_raster_data = rgba_raster_data(&frame, global_color_map.as_ref());
            frame.raster_data = rgba_raster_data;
            frame.local_color_map = None;
            frame.image_descriptor.m = false;
        }
        screen_descriptor.m = false;
        global_color_map = None;
    }

    Ok(Gif {
        signature: signature.to_string(),
        screen_descriptor,
        global_color_map,
        frames,
    })
}

fn rgba_raster_data(
    frame: &Frame,
    global_color_map: Option<&ColorMap>,
) -> Vec<u8> {
    let color_map = if frame.image_descriptor.m {
        frame
            .local_color_map
            .as_ref()
            .expect("expected local color map not present")
    } else {
        global_color_map.expect("expected global color map not present")
    };

    frame
        .raster_data
        .iter()
        .map(|index| {
            table_index_to_rgba(
                *index,
                color_map,
                frame
                    .graphic_control_extension
                    .as_ref()
                    .and_then(|ext| ext.transparent_color_index),
            )
        })
        .flatten()
        .collect()
}

fn table_index_to_rgba(
    index: u8,
    color_map: &ColorMap,
    maybe_transparent_color_index: Option<u8>,
) -> Vec<u8> {
    let rgba = color_map
        .get(&(index as usize))
        .expect("pixel index not found in color map");
    let alpha = maybe_transparent_color_index
        .filter(|alpha_index| index == *alpha_index)
        .map(|_| 0x00u8)
        .unwrap_or(0xFFu8);
    vec![rgba.r, rgba.g, rgba.b, alpha]
}

fn frames(bytes: &Vec<u8>, cursor: usize) -> (Vec<Frame>, usize) {
    let mut mut_index = cursor;
    let mut frames: Vec<Frame> = Vec::new();

    while bytes[mut_index] != 0x3b {
        let (frame, index) = frame(bytes, mut_index);
        mut_index = index;
        frames.push(frame);
    }

    (frames, mut_index)
}

fn frame(bytes: &Vec<u8>, cursor: usize) -> (Frame, usize) {
    let mut index = cursor;
    let mut graphic_control_extension: Option<GraphicControlExtension> = None;

    while bytes[index] != 0x2c {
        if bytes[index] == 0x21 {
            let (block, cursor) = steps::extension_block::decode(bytes, index);
            index = cursor;

            match block {
                Some(ExtensionBlock::GraphicControlExtension(extension)) => {
                    graphic_control_extension = Some(extension);
                }
                _ => {}
            }
        } else {
            index += 1;
        }
    }

    let (image_descriptor, index) = steps::image_descriptor::decode(bytes, index);
    let (color_map, index) =
        steps::color_map::decode(bytes, image_descriptor.pixel, image_descriptor.m, index);
    let (raster_data, index) = steps::raster_data::decode(bytes, index);

    (
        Frame {
            image_descriptor,
            local_color_map: color_map,
            raster_data,
            graphic_control_extension,
        },
        index,
    )
}

#[test]
pub fn should_decode() {
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    assert_eq!("GIF89a", gif.signature);
    assert_eq!(106, gif.frames.len());
    gif.frames.iter().for_each(|frame| {
        assert_eq!(
            frame.raster_data.len(),
            (frame.image_descriptor.image_width as u32 * frame.image_descriptor.image_height as u32)
                as usize
        );

        if frame.image_descriptor.m {
            assert!(frame.local_color_map.is_some())
        } else {
            assert!(frame.local_color_map.is_none())
        }
    });
}
