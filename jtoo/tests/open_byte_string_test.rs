use jtoo::{escape_ascii, Decoder, EncodeError, Encoder, ErrorReason};

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

#[test]
fn consume() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedByteString)),
        (b"B0".as_slice(), Err(ErrorReason::MalformedByteString)),
        (b"B0g".as_slice(), Err(ErrorReason::MalformedByteString)),
        (
            b"BA0".as_slice(),
            Err(ErrorReason::UppercaseHexNotAllowedInByteString),
        ),
        (
            b"B0A".as_slice(),
            Err(ErrorReason::UppercaseHexNotAllowedInByteString),
        ),
        (b"B".as_slice(), Ok(vec![])),
        (b"B00".as_slice(), Ok(vec![0])),
        (b"Bff".as_slice(), Ok(vec![0xff])),
        (
            b"Bf0e1d2c3b4a5968778695a4b3c2d1e0f".as_slice(),
            Ok(vec![
                0xf0, 0xe1, 0xd2, 0xc3, 0xb4, 0xa5, 0x96, 0x87, 0x78, 0x69, 0x5a, 0x4b, 0x3c, 0x2d,
                0x1e, 0x0f,
            ]),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_byte_string();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}",);
                decoder.close().expect(&msg);
            }
            Err(reason) => assert_eq!(result.expect_err(&msg).reason, reason),
        }
    }
}
