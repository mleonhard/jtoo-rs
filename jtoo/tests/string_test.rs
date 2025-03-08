use jtoo::{escape_ascii, Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_string("a"), Err(EncodeError::UnclosedString));

    let mut encoder = Encoder::new();
    encoder.append_string("").unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""""#));

    let mut encoder = Encoder::new();
    encoder.append_string("string1").unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""string1""#));

    let mut encoder = Encoder::new();
    encoder.append_string("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f").unwrap();
    assert_eq!(
        encoder.as_str(),
        Ok(
            r#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#
        )
    );
}

#[test]
fn append_in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_string("a").unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#"["a",T]"#));
}

#[test]
fn append_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn consume() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedString)),
        (b"\"", Err(ErrorReason::UnclosedString)),
        (
            b"\"abc",
            Err(ErrorReason::UnclosedString),
        ),
        (b"\"abc\"", Ok("abc".to_string())),
        (&[b'"', 0xe4, 0xbd, 0xa0, b'"'], Ok("你".to_string())),
        (
            &[b'"', 0xe4, 0xbd, b'"'],
            Err(ErrorReason::NotUtf8),
        ),
        (br#""\""#, Err(ErrorReason::IncompleteEscapeSequence)),
        (br#""\0""#, Err(ErrorReason::IncompleteEscapeSequence)),
        (br#""\g0""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\0g""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\20""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\21""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\5b""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\5d""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\7e""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\80""#, Err(ErrorReason::InvalidEscapeSequence)),
        (br#""\ff""#, Err(ErrorReason::InvalidEscapeSequence)),
        (
            br#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#,
            Ok("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f".to_string()),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_string();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}", );
                decoder.close().expect(&msg);
            }
            Err(reason) => assert_eq!(result.expect_err(&msg).reason, reason, "{msg}"),
        }
    }
}

#[test]
fn decode() {
    assert_eq!(String::decode(br#""a""#).unwrap().as_str(), "a");
    assert_eq!(<Box<str>>::decode(br#""a""#).unwrap().as_ref(), "a");
    assert_eq!(
        <Vec<String>>::decode(br#"["a","abc"]"#).unwrap(),
        vec!["a".to_string(), "abc".to_string()]
    );
}

#[test]
fn encode() {
    assert_eq!("abc".encode().unwrap().as_str(), r#""abc""#);
    assert_eq!("abc".to_string().encode().unwrap().as_str(), r#""abc""#);
    assert_eq!(
        "abc"
            .to_string()
            .into_boxed_str()
            .encode()
            .unwrap()
            .as_str(),
        r#""abc""#
    );
}
