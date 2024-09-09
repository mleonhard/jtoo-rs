use jtoo::{escape_ascii, DecodeError, Decoder};

#[test]
fn empty() {
    let decoder = Decoder::new(b"");
    decoder.close().unwrap();
}

#[test]
fn consume_bool() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedBool(vec![]))),
        (b"\"a\"", Err(DecodeError::ExpectedBool(b"\"a\"".to_vec()))),
        (b"!", Err(DecodeError::ExpectedBool(b"!".to_vec()))),
        (b"T", Ok(true)),
        (b"F", Ok(false)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(decoder.consume_bool(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_integer() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedInteger(vec![]))),
        (
            b"\"a\"",
            Err(DecodeError::ExpectedInteger(b"\"a\"".to_vec())),
        ),
        (b"T", Err(DecodeError::ExpectedInteger(b"T".to_vec()))),
        (b"!", Err(DecodeError::ExpectedInteger(b"!".to_vec()))),
        (b"0", Ok(0)),
        (b"1", Ok(1)),
        (b"10", Ok(10)),
        (b"100", Ok(100)),
        (b"1_000", Ok(1000)),
        (b"10_000", Ok(10000)),
        (b"100_000", Ok(100000)),
        (b"1_000_000", Ok(1000000)),
        (b"-1_000_000", Ok(-1000000)),
        (b"-100_000", Ok(-100000)),
        (b"-10_000", Ok(-10000)),
        (b"-1_000", Ok(-1000)),
        (b"-100", Ok(-100)),
        (b"-10", Ok(-10)),
        (b"-1", Ok(-1)),
        (b"9_223_372_036_854_775_807", Ok(i64::MAX)),
        (b"-0", Err(DecodeError::NegativeZero(b"-0".to_vec()))),
        (
            b"-00",
            Err(DecodeError::ExtraLeadingZeroes(b"-00".to_vec())),
        ),
        (b"00", Err(DecodeError::ExtraLeadingZeroes(b"00".to_vec()))),
        (
            b"1_",
            Err(DecodeError::IncorrectDigitGrouping(b"1_".to_vec())),
        ),
        (
            b"1_0",
            Err(DecodeError::IncorrectDigitGrouping(b"1_0".to_vec())),
        ),
        (
            b"1_00",
            Err(DecodeError::IncorrectDigitGrouping(b"1_00".to_vec())),
        ),
        (
            b"-1_00",
            Err(DecodeError::IncorrectDigitGrouping(b"-1_00".to_vec())),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(decoder.consume_integer(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_string() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedString(vec![]))),
        (b"\"", Err(DecodeError::UnclosedString(b"\"".to_vec()))),
        (
            b"\"abc",
            Err(DecodeError::UnclosedString(b"\"abc".to_vec())),
        ),
        (b"\"abc\"", Ok("abc".to_string())),
        (&[b'"', 0xe4, 0xbd, 0xa0, b'"'], Ok("你".to_string())),
        (
            &[b'"', 0xe4, 0xbd, b'"'],
            Err(DecodeError::NotUtf8(b"\"\xe4\xbd\"".to_vec())),
        ),
        (b"\"\\\"", Err(DecodeError::IncompleteEscape(b"\\\"".to_vec()))),
        (b"\"\\0\"", Err(DecodeError::IncompleteEscape(b"\\0\"".to_vec()))),
        (b"\"\\g0\"", Err(DecodeError::InvalidEscape(b"\\g0".to_vec()))),
        (b"\"\\0g\"", Err(DecodeError::InvalidEscape(b"\\0g".to_vec()))),
        (b"\"\\20\"", Err(DecodeError::InvalidEscape(b"\\20".to_vec()))),
        (b"\"\\21\"", Err(DecodeError::InvalidEscape(b"\\21".to_vec()))),
        (b"\"\\5b\"", Err(DecodeError::InvalidEscape(b"\\5b".to_vec()))),
        (b"\"\\5d\"", Err(DecodeError::InvalidEscape(b"\\5d".to_vec()))),
        (b"\"\\7e\"", Err(DecodeError::InvalidEscape(b"\\7e".to_vec()))),
        (b"\"\\80\"", Err(DecodeError::InvalidEscape(b"\\80".to_vec()))),
        (b"\"\\ff\"", Err(DecodeError::InvalidEscape(b"\\ff".to_vec()))),
        (
            br#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#,
            Ok("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f".to_string()),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(decoder.consume_string(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn list_missing_separator() {
    let mut decoder = Decoder::new(b"[TT]");
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_bool(),
        Err(DecodeError::ExpectedListSeparator(b"T]".to_vec()))
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_close_list(),
        Err(DecodeError::ExpectedListSeparator(b"[]]".to_vec()))
    );
}

#[test]
fn list_nested() {
    let mut decoder = Decoder::new(b"[[],[],[[T]]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(decoder.consume_bool(), Ok(true));
    decoder.consume_close_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_string_string() {
    let mut decoder = Decoder::new(b"[\"a\",\"b\"]");
    decoder.consume_open_list().unwrap();
    assert_eq!(decoder.consume_string(), Ok("a".to_string()));
    assert_eq!(decoder.consume_string(), Ok("b".to_string()));
    decoder.consume_close_list().unwrap();
    decoder.close().unwrap();
}
