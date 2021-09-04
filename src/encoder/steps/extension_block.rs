use crate::types::GraphicControlExtension;
use std::{u8, usize};

pub fn encode_graphic_control_extension(
    bytes: &mut [u8],
    graphic_control_extension: &GraphicControlExtension,
    cursor: usize,
) -> usize {
    bytes[cursor] = 0x21;
    bytes[cursor + 1] = 0xF9;
    bytes[cursor + 2] = 4;

    let mut flags: u8 = 0;
    flags ^= (graphic_control_extension.disposal_method() as u8) << 2;
    flags ^= (graphic_control_extension.user_input() as u8) << 1;
    flags ^= graphic_control_extension.transparent_color() as u8;
    bytes[cursor + 3] = flags;

    let delay_time_bytes = graphic_control_extension.delay_time().to_le_bytes();
    bytes[cursor + 4] = delay_time_bytes[0];
    bytes[cursor + 5] = delay_time_bytes[1];

    let mut block_terminator_offsset = 0;
    if let Some(index) = graphic_control_extension.transparent_color_index() {
        block_terminator_offsset = 1;
        bytes[cursor + 6] = index;
    }

    cursor + 6 + block_terminator_offsset
}

#[test]
pub fn should_encode() {
    use crate::decoder::decode;
    use crate::decoder::ColorOutput;
    use std::{fs::File, io::Read, path::Path};

    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    let source_bytes: &mut Vec<u8> = &mut Vec::new();
    File::open(Path::new("./ascii-gif-example.gif"))
        .unwrap()
        .read_to_end(source_bytes)
        .unwrap();

    // first frame has graphic control extension block
    let test_frame = &gif.frames()[0];
    // index of the first decoded graphic control extension block
    let from_index = 32;

    let mut bytes: [u8; 7] = [0; 7];

    encode_graphic_control_extension(
        &mut bytes,
        test_frame.graphic_control_extension().as_ref().unwrap(),
        0,
    );

    assert_eq!(&source_bytes[from_index..from_index + 7], bytes);
}
