use jtoo::{Encode, EncodeError, Encoder};

#[test]
fn encode() {
    assert_eq!(true.encode().unwrap().as_str(), "T");
    assert_eq!(false.encode().unwrap().as_str(), "F");
}

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.append_bool(true).unwrap();
    assert_eq!(encoder.as_str(), Ok("T"));

    let mut encoder = Encoder::new();
    encoder.append_bool(false).unwrap();
    assert_eq!(encoder.as_str(), Ok("F"));
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(false), Err(EncodeError::UnclosedString));
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.append_integer(1).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[T,1]"));
}
