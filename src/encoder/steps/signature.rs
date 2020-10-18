use crate::encoder;

const SIGNATURE_BYTES_LENGTH: usize = 6;

pub fn encode(
    bytes: &mut Vec<u8>,
    signature: &String,
    cursor: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let signature_bytes: Vec<u8> = signature.bytes().collect();

    if signature_bytes.len() != SIGNATURE_BYTES_LENGTH {
        return Err(Box::new(encoder::Error::InvalidGifSignature));
    }

    signature_bytes[cursor..cursor + SIGNATURE_BYTES_LENGTH]
        .iter()
        .enumerate()
        .for_each(|(index, byte)| {
            bytes.insert(index, *byte);
        });

    Ok(())
}

#[test]
pub fn should_encode() {
    let mut bytes = vec![];
    let signature = "GIF89a".to_string();
    encode(&mut bytes, &signature, 0).unwrap();

    assert_eq!([71, 73, 70, 56, 57, 97], bytes[0..SIGNATURE_BYTES_LENGTH]);
}
