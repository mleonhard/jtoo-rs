use jtoo::{EncodeError, Encoder};

#[test]
fn empty() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.as_str(), Ok("B"));
}

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder
        .append_byte_string(&[
            0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
            0xe1, 0xf0,
        ])
        .unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.as_str(), Ok("B0f1e2d3c4b5a69788796a5b4c3d2e1f0"));
}

#[test]
fn append_twice() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder.append_byte_string(&[0x0f, 0x1e]).unwrap();
    encoder.append_byte_string(&[0x2d, 0x3c]).unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.as_str(), Ok("B0f1e2d3c"));
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_byte_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_byte_string().unwrap();
    encoder.close_byte_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[B,T]"));
}

#[test]
fn unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    assert_eq!(
        encoder.append_bool(true),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedByteString));
}

#[test]
fn not_in_byte_string() {
    let mut encoder = Encoder::new();
    assert_eq!(
        encoder.append_byte_string(&[0x0]),
        Err(EncodeError::NotInByteString)
    );
    assert_eq!(
        encoder.close_byte_string(),
        Err(EncodeError::NotInByteString)
    );
}
