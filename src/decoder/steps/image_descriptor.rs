use crate::decoder::steps::nth_bit;
use crate::types::ImageDescriptor;

pub fn decode(bytes: &Vec<u8>, cursor: usize) -> (ImageDescriptor, usize) {
    let to_index = cursor + 10;
    let image_descriptor = &bytes[cursor..to_index];

    let image_left = ((image_descriptor[2] as u16) << 8) | image_descriptor[1] as u16;
    let image_top = ((image_descriptor[4] as u16) << 8) | image_descriptor[3] as u16;
    let image_width = ((image_descriptor[6] as u16) << 8) | image_descriptor[5] as u16;
    let image_height = ((image_descriptor[8] as u16) << 8) | image_descriptor[7] as u16;

    let flags: u8 = image_descriptor[9];
    let m: bool = nth_bit(flags, 7);
    let i: bool = nth_bit(flags, 6);
    let pixel: u8 = (flags << 5) >> 5;

    (
        ImageDescriptor::new(
            image_left,
            image_top,
            image_width,
            image_height,
            m,
            i,
            pixel,
        ),
        to_index,
    )
}
