use crate::types::ScreenDescriptor;
use std::{u8, usize};

pub fn encode(
    bytes: &mut Vec<u8>,
    screen_descriptor: &ScreenDescriptor,
    cursor: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let width_bytes = screen_descriptor.width().to_le_bytes();
    bytes[cursor] = width_bytes[0];
    bytes[cursor + 1] = width_bytes[1];
    let height_bytes = screen_descriptor.height().to_le_bytes();
    bytes[cursor + 2] = height_bytes[0];
    bytes[cursor + 3] = height_bytes[1];

    let mut flags_byte: u8 = (screen_descriptor.m() as u8) << 7;
    flags_byte ^= screen_descriptor.cr() << 4;
    flags_byte ^= (screen_descriptor.sort() as u8) << 3;
    flags_byte ^= screen_descriptor.pixel();
    bytes[cursor + 4] = flags_byte;

    bytes[cursor + 5] = screen_descriptor.background();
    bytes[cursor + 6] = screen_descriptor.aspect_ratio();

    Ok((cursor + 7) as usize)
}

#[test]
pub fn should_encode() {
    use crate::decoder::{decode, ColorOutput};
    use std::{fs::File, io::Read, path::Path};

    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    let mut bytes: Vec<u8> = [0; 7].to_vec();
    encode(&mut bytes, &gif.screen_descriptor(), 0).unwrap();

    let source_bytes: &mut Vec<u8> = &mut Vec::new();
    File::open(Path::new("./ascii-gif-example.gif"))
        .unwrap()
        .read_to_end(source_bytes)
        .unwrap();

    assert_eq!(&source_bytes[6..13], bytes);
}
