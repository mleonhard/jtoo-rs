use jtoo::{Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

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
fn append_in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.append_integer(1).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[T,1]"));
}

#[test]
fn append_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(false), Err(EncodeError::UnclosedString));
}

#[test]
fn consume() {
    let mut decoder = Decoder::new(b"");
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedBool
    );

    let mut decoder = Decoder::new(br#""a""#);
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedBool
    );

    let mut decoder = Decoder::new(b"!");
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedBool
    );

    let mut decoder = Decoder::new(b"TT");
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::MalformedBool
    );

    let mut decoder = Decoder::new(b"T");
    assert!(decoder.consume_bool().unwrap());
    decoder.close().unwrap();

    let mut decoder = Decoder::new(b"F");
    assert!(!decoder.consume_bool().unwrap());
    decoder.close().unwrap();

    let mut decoder = Decoder::new(b"[T,1]");
    decoder.consume_list_open().unwrap();
    assert!(decoder.consume_bool().unwrap());
    assert_eq!(decoder.consume_integer(), Ok(1));
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn decode() {
    assert!(bool::decode(b"T").unwrap());
    assert!(!bool::decode(b"F").unwrap());
}

#[test]
fn encode() {
    assert_eq!(true.encode().unwrap().as_str(), "T");
    assert_eq!(false.encode().unwrap().as_str(), "F");
}
