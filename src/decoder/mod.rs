use crate::types::{ColorMap, ExtensionBlock, Frame, Gif, GraphicControlExtension};
use std::io::Read;
#[cfg(test)]
use {std::fs::File, std::path::Path};

mod steps;

/// Color output mode.
#[derive(PartialEq, Debug)]
pub enum ColorOutput {
    /// Every byte of the raster data is expanded to 4 bytes (R G B A).
    /// Setting this color output the rgba_raster_data will be present in the resulting Gif.
    RGBA,
    /// Normal ColorMap index color mapping.
    ColorMap,
}

/// Decode a gif encoded source.
/// If RGBA ColorOutput is set, the rgba_raster_data is set in the resulting Gif.
pub fn decode(
    mut source: impl Read,
    color_output: ColorOutput,
) -> Result<Gif, Box<dyn std::error::Error>> {
    let bytes: &mut Vec<u8> = &mut Vec::new();
    source.read_to_end(bytes)?;

    let (signature, cursor) = steps::signature::decode(&bytes, 0);
    let (screen_descriptor, cursor) = steps::screen_descriptor::decode(&bytes, cursor);
    let (global_color_map, cursor) = steps::color_map::decode(
        &bytes,
        screen_descriptor.pixel(),
        screen_descriptor.m(),
        cursor,
    );
    let (frames, _cursor) = frames(bytes, &color_output, global_color_map.as_ref(), cursor);

    Ok(Gif::new(
        signature,
        screen_descriptor,
        global_color_map,
        frames,
    ))
}

fn rgba_raster_data(
    raster_data: &[u8],
    graphic_control_extension: Option<&GraphicControlExtension>,
    color_map: &ColorMap,
) -> Vec<u8> {
    raster_data
        .iter()
        .map(|index| {
            table_index_to_rgba(
                *index,
                color_map,
                graphic_control_extension.and_then(|ext| ext.transparent_color_index()),
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
    vec![rgba.r(), rgba.g(), rgba.b(), alpha]
}

fn frames(
    bytes: &[u8],
    color_output: &ColorOutput,
    global_color_map: Option<&ColorMap>,
    cursor: usize,
) -> (Vec<Frame>, usize) {
    let mut mut_index = cursor;
    let mut frames: Vec<Frame> = Vec::new();

    while bytes[mut_index] != 0x3b {
        let (frame, index) = frame(bytes, color_output, global_color_map, mut_index);
        mut_index = index;
        frames.push(frame);
    }

    (frames, mut_index)
}

fn frame(
    bytes: &[u8],
    color_output: &ColorOutput,
    global_color_map: Option<&ColorMap>,
    cursor: usize,
) -> (Frame, usize) {
    let mut index = cursor;
    let mut graphic_control_extension: Option<GraphicControlExtension> = None;

    while bytes[index] != 0x2c {
        if bytes[index] == 0x21 {
            let (block, cursor) = steps::extension_block::decode(bytes, index);
            index = cursor;

            if let Some(ExtensionBlock::GraphicControlExtension(extension)) = block {
                graphic_control_extension = Some(extension);
            }
        } else {
            // skip unhandled extension block
            index += 1;
        }
    }

    let (image_descriptor, index) = steps::image_descriptor::decode(bytes, index);
    let (color_map, index) =
        steps::color_map::decode(bytes, image_descriptor.pixel(), image_descriptor.m(), index);
    let (raster_data, index) = steps::raster_data::decode(bytes, index);

    let rgba_rd = match color_output {
        ColorOutput::RGBA => {
            let color_map = if image_descriptor.m() {
                color_map
                    .as_ref()
                    .expect("expected local color map not present")
            } else {
                global_color_map
                    .as_ref()
                    .expect("expected global color map not present")
            };

            Some(rgba_raster_data(
                &raster_data,
                graphic_control_extension.as_ref(),
                color_map,
            ))
        }
        ColorOutput::ColorMap => None,
    };

    (
        Frame::new(
            image_descriptor,
            color_map,
            raster_data,
            rgba_rd,
            graphic_control_extension,
        ),
        index,
    )
}

#[test]
pub fn should_decode_using_color_map_mode() {
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    assert_eq!("GIF89a", gif.signature());
    assert_eq!(106, gif.frames().len());
    gif.frames().iter().for_each(|frame| {
        assert_eq!(
            frame.raster_data().len(),
            (frame.image_descriptor().image_width() as u32
                * frame.image_descriptor().image_height() as u32) as usize
        );
        assert_eq!(&None, frame.rgba_raster_data());
        assert!(frame.local_color_map().is_some())
    });
}

#[test]
pub fn should_decode_using_rgba_mode() {
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::RGBA).unwrap();

    assert_eq!("GIF89a", gif.signature());
    assert_eq!(106, gif.frames().len());
    gif.frames().iter().for_each(|frame| {
        assert_eq!(
            frame.raster_data().len(),
            (frame.image_descriptor().image_width() as u32
                * frame.image_descriptor().image_height() as u32) as usize
        );
        assert_eq!(
            frame.rgba_raster_data().as_ref().unwrap().len(),
            (frame.image_descriptor().image_width() as u32
                * frame.image_descriptor().image_height() as u32
                * 4) as usize
        );

        if frame.image_descriptor().m() {
            assert!(frame.local_color_map().is_some())
        } else {
            assert!(frame.local_color_map().is_none())
        }

        assert!(frame.graphic_control_extension().is_some())
    });
}
