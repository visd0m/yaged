use std::str::from_utf8;

pub fn decode(bytes: &[u8], cursor: usize) -> (String, usize) {
    let to_index = cursor + 6;
    let signature = from_utf8(&bytes[cursor..to_index]).unwrap();
    (signature.to_string(), to_index)
}

#[test]
pub fn should_decode() {
    let bytes = vec![71, 73, 70, 56, 57, 97];

    let (decoded, cursor) = decode(&bytes, 0);

    assert_eq!("GIF89a".to_string(), decoded);
    assert_eq!(6, cursor);
}
