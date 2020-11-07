use crate::decoder::steps::nth_bit;
use crate::types::ScreenDescriptor;

pub fn decode(bytes: &[u8], cursor: usize) -> (ScreenDescriptor, usize) {
    let to_index = cursor + 7;
    let screen_descriptor = &bytes[cursor..to_index];

    let screen_width = ((screen_descriptor[1] as u16) << 8) | screen_descriptor[0] as u16;
    let screen_height = ((screen_descriptor[3] as u16) << 8) | screen_descriptor[2] as u16;

    let flags: u8 = screen_descriptor[4];
    let m: bool = nth_bit(flags, 7);
    let cr: u8 = (flags << 1) >> 5;
    let pixel: u8 = (flags << 5) >> 5;

    let background: u8 = screen_descriptor[5];

    (
        ScreenDescriptor::new(screen_width, screen_height, m, cr, pixel, background),
        to_index,
    )
}
