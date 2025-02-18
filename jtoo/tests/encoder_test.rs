use jtoo::{EncodeError, Encoder};

#[test]
fn empty() {
    let encoder = Encoder::new();
    assert_eq!(encoder.as_str(), Err(EncodeError::Empty));
}

#[test]
fn as_str() {
    let mut encoder = Encoder::new();
    encoder.append_bool(true).unwrap();
    assert_eq!(encoder.as_str(), Ok("T"));
}

#[test]
fn into_string() {
    let mut encoder = Encoder::new();
    encoder.append_bool(true).unwrap();
    assert_eq!(encoder.into_string(), Ok("T".to_string()));
}
