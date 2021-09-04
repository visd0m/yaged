//! Gif encoder/decoder bases on [GIF89a specification](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).
//!
//! #### Decoding
//! ```
//! use {std::fs::File, std::path::Path};
//!
//! // decodes a gif using ColorMap ColorOutput mode
//! let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
//! let color_map_gif = yaged::decoder::decode(file, yaged::decoder::ColorOutput::ColorMap).unwrap();
//! color_map_gif.frames().iter().for_each(|frame| {
//!  assert!(frame.rgba_raster_data().is_none())
//! });
//!
//! // decodes a gif using RGBA ColorOutput mode
//! let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
//! let rgba_gif = yaged::decoder::decode(file, yaged::decoder::ColorOutput::RGBA).unwrap();
//! // with this color output mode the rgba_raster_data() will be present in each frame
//! rgba_gif.frames().iter().for_each(|frame| {
//!  assert!(frame.rgba_raster_data().is_some())
//! });
//! ```

pub mod decoder;
pub mod encoder;
pub mod types;
