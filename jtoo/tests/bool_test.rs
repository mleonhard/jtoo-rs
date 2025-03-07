use jtoo::{escape_ascii, Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

// TODO: Add missing tests.  Organize tests better.

#[test]
fn encode() {
    assert_eq!(true.encode().unwrap().as_str(), "T");
    assert_eq!(false.encode().unwrap().as_str(), "F");
}

#[test]
fn decode() {
    assert!(bool::decode(b"T").unwrap());
    assert!(!bool::decode(b"F").unwrap());
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
fn consume() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedBool)),
        (b"\"a\"", Err(ErrorReason::ExpectedBool)),
        (b"!", Err(ErrorReason::ExpectedBool)),
        (b"TT", Err(ErrorReason::MalformedBool)),
        (b"T", Ok(true)),
        (b"F", Ok(false)),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_bool();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(reason) => assert_eq!(result.expect_err(&msg).reason, reason),
        }
    }
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
