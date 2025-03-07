use jtoo::{escape_ascii, Decoder, Encode, EncodeError, Encoder, ErrorReason};

#[test]
fn encode() {
    assert_eq!((-42i8).encode().unwrap().as_str(), "-42");
    assert_eq!((-42i16).encode().unwrap().as_str(), "-42");
    assert_eq!((-42i32).encode().unwrap().as_str(), "-42");
    assert_eq!((-42i64).encode().unwrap().as_str(), "-42");
    assert_eq!(42u8.encode().unwrap().as_str(), "42");
    assert_eq!(42u16.encode().unwrap().as_str(), "42");
    assert_eq!(42u32.encode().unwrap().as_str(), "42");
    assert_eq!(42u64.encode().unwrap().as_str(), "42");
}

#[test]
fn append() {
    for (value, expected) in [
        (0, "0"),
        (1, "1"),
        (10, "10"),
        (100, "100"),
        (1_000, "1_000"),
        (10_000, "10_000"),
        (100_000, "100_000"),
        (1_000_000, "1_000_000"),
        (-1_000_000, "-1_000_000"),
        (-100_000, "-100_000"),
        (-10_000, "-10_000"),
        (-1_000, "-1_000"),
        (-100, "-100"),
        (-10, "-10"),
        (-1, "-1"),
        (i64::MIN, "-9_223_372_036_854_775_808"),
        (i64::MAX, "9_223_372_036_854_775_807"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_integer(value).unwrap();
        assert_eq!(encoder.as_str(), Ok(expected.to_string().as_str()));
    }
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_integer(1), Err(EncodeError::UnclosedString));
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_integer(1).unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[1,T]"));
}

#[test]
fn consume_integer() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedInteger)),
        (b"\"a\"", Err(ErrorReason::ExpectedInteger)),
        (b"Y", Err(ErrorReason::ExpectedInteger)),
        (b"!", Err(ErrorReason::ExpectedInteger)),
        (b"-", Err(ErrorReason::ExpectedInteger)),
        (b"-0", Err(ErrorReason::NegativeZero)),
        (b"0", Ok(0)),
        (b"1", Ok(1)),
        (b"12", Ok(12)),
        (b"123", Ok(123)),
        (b"1_234", Ok(1_234)),
        (b"12_345", Ok(12_345)),
        (b"123_456", Ok(123_456)),
        (b"1_234_567", Ok(1_234_567)),
        (b"-1_234_567", Ok(-1_234_567)),
        (b"-123_456", Ok(-123_456)),
        (b"-12_345", Ok(-12_345)),
        (b"-1_234", Ok(-1_234)),
        (b"-123", Ok(-123)),
        (b"-12", Ok(-12)),
        (b"-1", Ok(-1)),
        (b"9_223_372_036_854_775_807", Ok(i64::MAX)),
        (
            b"9_223_372_036_854_775_808",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (b"-9_223_372_036_854_775_808", Ok(i64::MIN)),
        (
            b"-9_223_372_036_854_775_809",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (
            b"9_900_000_000_000_000_000",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (b"-0", Err(ErrorReason::NegativeZero)),
        (b"-00", Err(ErrorReason::ExpectedSingleZero)),
        (b"00", Err(ErrorReason::ExpectedSingleZero)),
        (b"1000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_1", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1__", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"-1_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0000_000", Err(ErrorReason::IncorrectDigitGrouping)),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_integer();
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
fn consume_unsigned_integer() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"\"a\"", Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"Y", Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"!", Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"-", Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"-1", Err(ErrorReason::ExpectedUnsignedInteger)),
        (b"0", Ok(0)),
        (b"1", Ok(1)),
        (b"12", Ok(12)),
        (b"123", Ok(123)),
        (b"1_234", Ok(1_234)),
        (b"12_345", Ok(12_345)),
        (b"123_456", Ok(123_456)),
        (b"1_234_567", Ok(1_234_567)),
        (b"18_446_744_073_709_551_615", Ok(u64::MAX)),
        (
            b"18_446_744_073_709_551_616",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (
            b"18_500_000_000_000_000_000",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (b"00", Err(ErrorReason::ExpectedSingleZero)),
        (b"1000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_1", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1__", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0000_000", Err(ErrorReason::IncorrectDigitGrouping)),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_unsigned_integer();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(reason) => assert_eq!(result.expect_err(&msg).reason, reason, "{msg}"),
        }
    }
}
