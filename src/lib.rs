//! Gif encoder/decoder bases on [GIF89a specification](https://www.w3.org/Graphics/GIF/spec-gif89a.txt).
//!
//! #### Decoding
//! ```rust
//! let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
//! let gif = decode(file, ColorOutput::ColorMap).unwrap();
//! ```

pub mod decoder;
pub mod types;
