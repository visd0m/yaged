//! Gif encoder/decoder bases on [GIF89a specification](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).
//!
//! #### Decoding
//! ```
//! use {std::fs::File, std::path::Path};
//!
//! let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
//! let gif = yaged::decoder::decode(file, yaged::decoder::ColorOutput::ColorMap).unwrap();
//! ```

pub mod decoder;
pub mod types;
