use crate::encoder::steps::signature;
use crate::types::Gif;
use std::fmt::{Display, Formatter};

use self::steps::screen_descriptor;
#[cfg(test)]
use {
    crate::decoder::{decode, ColorOutput},
    std::fs::File,
    std::io::Read,
    std::path::Path,
};

mod steps;

#[derive(Debug)]
pub enum Error {
    UnhandledInterlacedFlag,
    InvalidGifSignature,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

/// Encode a Gif in a bytes vec.
pub fn encode(gif: Gif) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut encoded: Vec<u8> = Vec::new();

    if gif
        .frames()
        .iter()
        .any(|frame| frame.image_descriptor().i())
    {
        return Err(Box::new(Error::UnhandledInterlacedFlag));
    }

    let mut cursor = signature::encode(&mut encoded, &gif.signature().to_string(), 0)?;
    cursor = screen_descriptor::encode(&mut encoded, &gif.screen_descriptor(), cursor)?;

    Ok(encoded)
}

#[test]
pub fn should_encode() {
    // todo when encoding is completed this test should work
    let bytes: &mut Vec<u8> = &mut Vec::new();
    let file = &mut File::open(Path::new("./ascii-gif-example.gif")).unwrap();
    let gif = decode(file, ColorOutput::ColorMap).unwrap();

    File::open(Path::new("./ascii-gif-example.gif"))
        .unwrap()
        .read_to_end(bytes)
        .unwrap();

    let encoded = encode(gif).unwrap();

    assert_eq!(bytes, &encoded)
}
