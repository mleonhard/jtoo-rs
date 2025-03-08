use jtoo::{
    escape_ascii, ByteString, Decimal, Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason,
};

#[test]
fn append() {
    let mut encoder = Encoder::new();
    assert_eq!(
        encoder.append_byte_string(&[0x0]),
        Err(EncodeError::NotInByteString)
    );
    assert_eq!(
        encoder.close_byte_string(),
        Err(EncodeError::NotInByteString)
    );

    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.as_str(), Ok("B"));

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

    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder.append_byte_string(&[0x0f, 0x1e]).unwrap();
    encoder.append_byte_string(&[0x2d, 0x3c]).unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.as_str(), Ok("B0f1e2d3c"));

    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    assert_eq!(
        encoder.append_bool(true),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(encoder.append_byte_string(b"abc"), Ok(()));
    assert_eq!(
        encoder.append_date_time_offset(1, 1, 1, 1, 1, 1, 1, 1, 1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_decimal(Decimal::ZERO),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_integer(1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_string("abc"),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_timestamp_seconds(1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_timestamp_milliseconds(1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_timestamp_microseconds(1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.append_timestamp_nanosecond(1),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(
        encoder.open_byte_string(),
        Err(EncodeError::UnclosedByteString)
    );
}

#[test]
fn append_in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_byte_string().unwrap();
    encoder.append_byte_string(&[0]).unwrap();
    encoder.close_byte_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[B00,T]"));
}

#[test]
fn append_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_byte_string(), Err(EncodeError::UnclosedString));
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

#[test]
fn decode() {
    assert_eq!(ByteString::decode(b"B").unwrap(), ByteString(vec![]));
    assert_eq!(
        ByteString::decode(b"B07020f").unwrap(),
        ByteString(vec![7, 2, 15])
    );
}

#[test]
fn encode() {
    assert_eq!(ByteString(vec![]).encode().unwrap().as_str(), "B");
    assert_eq!(
        ByteString(vec![7, 2, 15]).encode().unwrap().as_str(),
        "B07020f"
    );
}
