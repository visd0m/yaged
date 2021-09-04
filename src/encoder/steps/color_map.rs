use std::usize;

use crate::types::ColorMap;

pub fn encode(
    bytes: &mut Vec<u8>,
    color_map: Option<&ColorMap>,
    cursor: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    if let Some(color_map) = color_map {
        for (index, rgb) in color_map.values().enumerate() {
            bytes[cursor + index] = rgb.r();
            bytes[cursor + index + 1] = rgb.g();
            bytes[cursor + index + 2] = rgb.b();
        }

        Ok(cursor + color_map.len() * 3)
    } else {
        Ok(cursor)
    }
}

#[test]
pub fn should_encode() {
    use crate::decoder::{decode, ColorOutput};
    use std::{fs::File, io::Read, path::Path};

    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    let to_index = 3 * 2i32.pow(gif.screen_descriptor().pixel() as u32 + 1);

    let mut bytes: Vec<u8> = Vec::with_capacity(to_index as usize);
    encode(&mut bytes, gif.global_color_map().as_ref(), 0).unwrap();

    let source_bytes: &mut Vec<u8> = &mut Vec::new();
    File::open(Path::new("./ascii-gif-example.gif"))
        .unwrap()
        .read_to_end(source_bytes)
        .unwrap();

    assert_eq!(Vec::new() as Vec<u8>, bytes);
}

// todo add test with global colormap
