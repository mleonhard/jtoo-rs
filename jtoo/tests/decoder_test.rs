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
        (b"-0", Err(DecodeError::NegativeZero(b"-0".to_vec()))),
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
        (
            b"9_223_372_036_854_775_808",
            Err(DecodeError::IntegerTooLarge(
                b"9_223_372_036_854_775_808".to_vec(),
            )),
        ),
        (b"-9_223_372_036_854_775_808", Ok(i64::MIN)),
        (
            b"-9_223_372_036_854_775_809",
            Err(DecodeError::IntegerTooLarge(
                b"-9_223_372_036_854_775_809".to_vec(),
            )),
        ),
        (b"-0", Err(DecodeError::NegativeZero(b"-0".to_vec()))),
        (
            b"-00",
            Err(DecodeError::ExtraLeadingZeroes(b"-00".to_vec())),
        ),
        (b"00", Err(DecodeError::ExtraLeadingZeroes(b"00".to_vec()))),
        (
            b"1000",
            Err(DecodeError::IncorrectDigitGrouping(b"1000".to_vec())),
        ),
        (
            b"_",
            Err(DecodeError::IncorrectDigitGrouping(b"_".to_vec())),
        ),
        (
            b"1_",
            Err(DecodeError::IncorrectDigitGrouping(b"1_".to_vec())),
        ),
        (
            b"_1",
            Err(DecodeError::IncorrectDigitGrouping(b"_1".to_vec())),
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
        (
            b"1_0000",
            Err(DecodeError::IncorrectDigitGrouping(b"1_0000".to_vec())),
        ),
        (
            b"1_000_",
            Err(DecodeError::IncorrectDigitGrouping(b"1_000_".to_vec())),
        ),
        (
            b"1_000_0",
            Err(DecodeError::IncorrectDigitGrouping(b"1_000_0".to_vec())),
        ),
        (
            b"1_000_00",
            Err(DecodeError::IncorrectDigitGrouping(b"1_000_00".to_vec())),
        ),
        (
            b"1_000_0000",
            Err(DecodeError::IncorrectDigitGrouping(b"1_000_0000".to_vec())),
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
fn consume_year() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedDate(vec![]))),
        (b"T", Err(DecodeError::ExpectedDate(b"T".to_vec()))),
        (b"!", Err(DecodeError::ExpectedDate(b"!".to_vec()))),
        (b"D0", Err(DecodeError::MalformedDate(b"D0".to_vec()))),
        (b"D00", Err(DecodeError::MalformedDate(b"D00".to_vec()))),
        (b"D000", Err(DecodeError::MalformedDate(b"D000".to_vec()))),
        (
            b"D0000",
            Err(DecodeError::YearOutOfRange(b"D0000".to_vec())),
        ),
        (b"D0001", Ok(1u16)),
        (b"D2024", Ok(2024)),
        (b"D9999", Ok(9999)),
        (b"D999a", Err(DecodeError::MalformedDate(b"D999a".to_vec()))),
        (
            b"D10000",
            Err(DecodeError::MalformedDate(b"D10000".to_vec())),
        ),
        (
            b"D1000T",
            Err(DecodeError::MalformedDate(b"D1000T".to_vec())),
        ),
        (
            b"D2024!",
            Err(DecodeError::MalformedDate(b"D2024!".to_vec())),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(decoder.consume_year(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_month() {
    for (bytes, expected) in [
        (
            b"D2024".as_slice(),
            Err(DecodeError::ExpectedMonth(b"D2024".to_vec())),
        ),
        (
            b"D2024Z",
            Err(DecodeError::ExpectedMonth(b"D2024Z".to_vec())),
        ),
        (
            b"D2024-",
            Err(DecodeError::MalformedDate(b"D2024-".to_vec())),
        ),
        (
            b"D2024-0",
            Err(DecodeError::MalformedDate(b"D2024-0".to_vec())),
        ),
        (
            b"D2024-1",
            Err(DecodeError::MalformedDate(b"D2024-1".to_vec())),
        ),
        (
            b"D2024-1x",
            Err(DecodeError::MalformedDate(b"D2024-1x".to_vec())),
        ),
        (
            b"D2024-12x",
            Err(DecodeError::MalformedDate(b"D2024-12x".to_vec())),
        ),
        (
            b"D2024-00",
            Err(DecodeError::MonthOutOfRange(b"D2024-00".to_vec())),
        ),
        (b"D2024-01", Ok(1u8)),
        (b"D2024-12", Ok(12u8)),
        (
            b"D2024-13",
            Err(DecodeError::MonthOutOfRange(b"D2024-13".to_vec())),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        decoder.consume_year().unwrap();
        assert_eq!(decoder.consume_month(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_week() {
    for (bytes, expected) in [
        (
            b"D2024".as_slice(),
            Err(DecodeError::ExpectedWeek(b"D2024".to_vec())),
        ),
        (
            b"D2024Z",
            Err(DecodeError::ExpectedWeek(b"D2024Z".to_vec())),
        ),
        (
            b"D2024-",
            Err(DecodeError::MalformedDate(b"D2024-".to_vec())),
        ),
        (
            b"D2024-W",
            Err(DecodeError::MalformedDate(b"D2024-W".to_vec())),
        ),
        (
            b"D2024-W0",
            Err(DecodeError::MalformedDate(b"D2024-W0".to_vec())),
        ),
        (
            b"D2024-W1",
            Err(DecodeError::MalformedDate(b"D2024-W1".to_vec())),
        ),
        (
            b"D2024-W1x",
            Err(DecodeError::MalformedDate(b"D2024-W1x".to_vec())),
        ),
        (
            b"D2024-W12x",
            Err(DecodeError::MalformedDate(b"D2024-W12x".to_vec())),
        ),
        (
            b"D2024-W00",
            Err(DecodeError::WeekOutOfRange(b"D2024-W00".to_vec())),
        ),
        (b"D2024-W01", Ok(1u8)),
        (b"D2024-W53", Ok(53u8)),
        (
            b"D2024-W54",
            Err(DecodeError::WeekOutOfRange(b"D2024-W54".to_vec())),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        decoder.consume_year().unwrap();
        assert_eq!(decoder.consume_week(), expected, "{}", msg);
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_day() {
    for (bytes, expected) in [
        (
            b"D2024-01".as_slice(),
            Err(DecodeError::ExpectedDay(b"D2024-01".to_vec())),
        ),
        (
            b"D2024-01Z",
            Err(DecodeError::ExpectedDay(b"D2024-01Z".to_vec())),
        ),
        (
            b"D2024-W01Z",
            Err(DecodeError::ExpectedDay(b"D2024-W01Z".to_vec())),
        ),
        (
            b"D2024-01-",
            Err(DecodeError::MalformedDate(b"D2024-01-".to_vec())),
        ),
        (
            b"D2024-W01-",
            Err(DecodeError::MalformedDate(b"D2024-W01-".to_vec())),
        ),
        (
            b"D2024-01-x",
            Err(DecodeError::MalformedDate(b"D2024-01-x".to_vec())),
        ),
        (
            b"D2024-01-0",
            Err(DecodeError::MalformedDate(b"D2024-01-0".to_vec())),
        ),
        (
            b"D2024-01-1",
            Err(DecodeError::MalformedDate(b"D2024-01-1".to_vec())),
        ),
        (
            b"D2024-01-1x",
            Err(DecodeError::MalformedDate(b"D2024-01-1x".to_vec())),
        ),
        (
            b"D2024-01-12x",
            Err(DecodeError::MalformedDate(b"D2024-01-12x".to_vec())),
        ),
        (
            b"D2024-01-00",
            Err(DecodeError::DayOutOfRange(b"D2024-01-00".to_vec())),
        ),
        (b"D2024-01-01", Ok(1u8)),
        (b"D2024-W01-01", Ok(1u8)),
        (b"D2024-01-31", Ok(31u8)),
        (b"D2024-W01-31", Ok(31u8)),
        (
            b"D2024-01-32",
            Err(DecodeError::DayOutOfRange(b"D2024-01-32".to_vec())),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        decoder.consume_year().unwrap();
        if bytes.contains(&b'W') {
            decoder.consume_week().unwrap();
        } else {
            decoder.consume_month().unwrap();
        }
        assert_eq!(decoder.consume_day(), expected, "{}", msg);
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
        Err(DecodeError::ExpectedListSeparator(b"[TT]".to_vec()))
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_close_list(),
        Err(DecodeError::ExpectedListSeparator(b"[[][]]".to_vec()))
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
