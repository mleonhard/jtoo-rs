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
    let encoder = Encoder::new();
    assert_eq!(encoder.into_string(), Err(EncodeError::Empty));

    let mut encoder = Encoder::new();
    encoder.append_bool(true).unwrap();
    assert_eq!(encoder.into_string(), Ok("T".to_string()));

    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedString));

    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedByteString));

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedList));

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedList));
}
