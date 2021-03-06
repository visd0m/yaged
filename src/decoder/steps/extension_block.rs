use crate::decoder::steps::nth_bit;
use crate::types::{ExtensionBlock, GraphicControlExtension};

pub fn decode(bytes: &[u8], cursor: usize) -> (Option<ExtensionBlock>, usize) {
    let label = bytes[cursor + 1];
    match label {
        0xf9 => {
            let (block, cursor) = graphic_control_extension(bytes, cursor + 2);
            (Some(ExtensionBlock::GraphicControlExtension(block)), cursor)
        }
        _ => (None, cursor + 2),
    }
}

pub fn graphic_control_extension(bytes: &[u8], cursor: usize) -> (GraphicControlExtension, usize) {
    let flags = bytes[cursor + 1];
    let disposal_method = (flags << 3) >> 5;
    let user_input_flag = nth_bit(flags, 1);
    let transparent_color_flag = nth_bit(flags, 0);

    let delay_time = ((bytes[cursor + 3] as u16) << 8) | bytes[cursor + 2] as u16;
    let transparent_color_index = if transparent_color_flag {
        Some(bytes[cursor + 4])
    } else {
        None
    };

    (
        GraphicControlExtension::new(
            disposal_method,
            user_input_flag,
            transparent_color_flag,
            delay_time,
            transparent_color_index,
        ),
        cursor + if transparent_color_flag { 6 } else { 5 },
    )
}
