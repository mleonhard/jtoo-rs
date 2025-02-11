use jtoo::{escape_ascii, DateTimeOffset, Decoder, ErrorReason};

#[test]
fn empty() {
    let decoder = Decoder::new(b"");
    decoder.close().unwrap();
}

#[test]
fn consume_bool() {
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
fn consume_byte_string() {
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
fn consume_string() {
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
#[allow(clippy::zero_prefixed_literal)]
#[allow(clippy::too_many_lines)]
fn consume_date_time_offset() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedDateTimeOffset)),
        (b"x", Err(ErrorReason::ExpectedDateTimeOffset)),
        (b"D", Err(ErrorReason::MalformedDate)),
        (b"D1", Err(ErrorReason::MalformedDate)),
        (b"D19", Err(ErrorReason::MalformedDate)),
        (b"D197", Err(ErrorReason::MalformedDate)),
        (b"D1970-", Err(ErrorReason::MalformedDate)),
        (b"D1970-0", Err(ErrorReason::MalformedDate)),
        (b"D1970-01", Err(ErrorReason::MalformedDate)),
        (b"D1970-01-", Err(ErrorReason::MalformedDate)),
        (b"D1970-01-0", Err(ErrorReason::MalformedDate)),
        (b"D1970-01-01", Err(ErrorReason::MalformedDateTimeOffset)),
        (b"D1970-01-01T", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T0", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:0", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:00", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:00:", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:00:0", Err(ErrorReason::MalformedTime)),
        (
            b"D1970-01-01T00:00:00",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (b"D1970-01-01T00:00:00Z", Ok(DateTimeOffset::UNIX_EPOCH)),
        (b"D1970-01-01T00:00:00.", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:00:00.0", Err(ErrorReason::MalformedTime)),
        (b"D1970-01-01T00:00:00.00", Err(ErrorReason::MalformedTime)),
        (
            b"D1970-01-01T00:00:00.000",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (b"D1970-01-01T00:00:00.000Z", Ok(DateTimeOffset::UNIX_EPOCH)),
        (
            b"D1970-01-01T00:00:00.000_",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_0",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_00",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_000",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (
            b"D1970-01-01T00:00:00.000_000Z",
            Ok(DateTimeOffset::UNIX_EPOCH),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_0",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_00",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_000",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_000Z",
            Ok(DateTimeOffset::UNIX_EPOCH),
        ),
        (b"D1970-01-01T00:00:00+", Err(ErrorReason::MalformedOffset)),
        (b"D1970-01-01T00:00:00-", Err(ErrorReason::MalformedOffset)),
        (b"D1970-01-01T00:00:00+0", Err(ErrorReason::MalformedOffset)),
        (b"D1970-01-01T00:00:00-0", Err(ErrorReason::MalformedOffset)),
        (
            b"D1970-01-01T00:00:00+00",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D1970-01-01T00:00:00-00",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D1970-01-01T00:00:00-0100",
            Err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted),
        ),
        (
            b"D1970-01-01T00:00:00+000",
            Err(ErrorReason::MalformedOffset),
        ),
        (
            b"D1970-01-01T00:00:00-000",
            Err(ErrorReason::MalformedOffset),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_000+0000",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D1970-01-01T00:00:00.000_000_000-0000",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D1234-05-06T07:08:09.123_456_768+0912",
            Ok(DateTimeOffset {
                year: 1234,
                month: 5,
                day: 6,
                hour: 7,
                minute: 8,
                second: 9,
                nanosecond: 123_456_768,
                offset_hour: 9,
                offset_minute: 12,
            }),
        ),
        (b"D0001-01-01T00:00:00-2359", Ok(DateTimeOffset::MIN)),
        (
            b"D9999-12-31T23:59:60.999_999_999+2359",
            Ok(DateTimeOffset::MAX),
        ),
        (b"D0000-01-01T00:00:00Z", Err(ErrorReason::YearOutOfRange)),
        (b"D10000-01-01T00:00:00Z", Err(ErrorReason::MalformedDate)),
        (b"D0001-00-01T00:00:00Z", Err(ErrorReason::MonthOutOfRange)),
        (b"D0001-13-01T00:00:00Z", Err(ErrorReason::MonthOutOfRange)),
        (b"D0001-001-01T00:00:00Z", Err(ErrorReason::MonthOutOfRange)),
        (b"D0001-01-00T00:00:00Z", Err(ErrorReason::DayOutOfRange)),
        (b"D0001-01-32T00:00:00Z", Err(ErrorReason::DayOutOfRange)),
        (b"D0001-01-001T00:00:00Z", Err(ErrorReason::DayOutOfRange)),
        (b"D0001-01-01T60:00:00Z", Err(ErrorReason::HourOutOfRange)),
        (b"D0001-01-01T001:00:00Z", Err(ErrorReason::MalformedTime)),
        (b"D0001-01-01T00:60:00Z", Err(ErrorReason::MinuteOutOfRange)),
        (b"D0001-01-01T00:001:00Z", Err(ErrorReason::MalformedTime)),
        (b"D0001-01-01T00:00:61Z", Err(ErrorReason::SecondOutOfRange)),
        (
            b"D0001-01-01T00:00:001Z",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (
            b"D0001-01-01T00:00:00.000_000_000_001Z",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (
            b"D0001-01-01T00:00:00+25",
            Err(ErrorReason::TimezoneOffsetHourOutOfRange),
        ),
        (
            b"D0001-01-01T00:00:00+0060",
            Err(ErrorReason::TimezoneOffsetMinuteOutOfRange),
        ),
        (
            b"D0001-01-01T00:00:01010",
            Err(ErrorReason::MalformedDateTimeOffset),
        ),
        (
            b"D0001-01-01T00:00:00+01Z",
            Err(ErrorReason::DataNotConsumed),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_date_time_offset();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = match result {
                    Ok(_) => decoder.close().expect_err(&msg),
                    Err(e) => e,
                };
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn list_missing_separator() {
    let mut decoder = Decoder::new(b"[TT]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );
}

#[test]
fn list_nested() {
    let mut decoder = Decoder::new(b"[[],[],[[T]]]");
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    assert_eq!(decoder.consume_bool(), Ok(true));
    decoder.consume_list_close().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_not_a_list() {
    assert_eq!(
        Decoder::new(b"T").consume_list_open().unwrap_err().reason,
        ErrorReason::ExpectedList
    );
}

#[test]
fn list_string_string() {
    let mut decoder = Decoder::new(b"[\"a\",\"b\"]");
    decoder.consume_list_open().unwrap();
    assert_eq!(decoder.consume_string(), Ok("a".to_string()));
    assert_eq!(decoder.consume_string(), Ok("b".to_string()));
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_has_next_item() {
    let mut decoder = Decoder::new(b"[T,T]");
    decoder.consume_list_open().unwrap();
    assert!(decoder.has_another_list_item());
    decoder.consume_bool().unwrap();
    assert!(decoder.has_another_list_item());
    decoder.consume_bool().unwrap();
    assert!(!decoder.has_another_list_item());
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn close_list_close_not_consumed() {
    let mut decoder = Decoder::new(b"[]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.close().unwrap_err().reason,
        ErrorReason::ListCloseNotConsumed
    );
}

#[test]
fn not_in_list() {
    let mut decoder = Decoder::new(b"T");
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::NotInList
    );
}

#[test]
fn expected_list_end() {
    let mut decoder = Decoder::new(b"[");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
}

#[test]
fn close_data_not_consumed() {
    assert_eq!(
        Decoder::new(b"Y").close().unwrap_err().reason,
        ErrorReason::DataNotConsumed
    );
}

#[test]
fn debug() {
    let _ = format!("{:?}", Decoder::new(b"x"));
}
