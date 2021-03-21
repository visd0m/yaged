use crate::types::{ColorMap, Rgb};
use std::collections::HashMap;

pub fn decode(bytes: &[u8], pixel: u8, m: bool, cursor: usize) -> (Option<ColorMap>, usize) {
    if m {
        let mut map = HashMap::new();

        let map_entries = 3 * 2i32.pow(pixel as u32 + 1);
        let to_index = cursor + map_entries as usize;
        let entries = &bytes[cursor..to_index];

        entries
            .chunks(3_usize)
            .map(|rgb| Rgb::new(rgb[0], rgb[1], rgb[2]))
            .enumerate()
            .for_each(|(index, rgb)| {
                map.insert(index, rgb);
            });

        (Some(map), to_index)
    } else {
        (None, cursor)
    }
}
