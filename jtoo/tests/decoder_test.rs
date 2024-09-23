use jtoo::{escape_ascii, Decoder, ErrorReason, Year, YearMonth, YearMonthDay, YearWeek};

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
        (b"T", Ok(true)),
        (b"F", Ok(false)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        match expected {
            Ok(expected_value) => assert_eq!(decoder.consume_bool(), Ok(expected_value), "{msg}"),
            Err(reason) => assert_eq!(decoder.consume_bool().expect_err(&msg).reason, reason),
        };
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_integer() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedInteger)),
        (b"\"a\"", Err(ErrorReason::ExpectedInteger)),
        (b"T", Err(ErrorReason::ExpectedInteger)),
        (b"!", Err(ErrorReason::ExpectedInteger)),
        (b"-0", Err(ErrorReason::NegativeZero)),
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
            Err(ErrorReason::IntegerTooLarge),
        ),
        (b"-9_223_372_036_854_775_808", Ok(i64::MIN)),
        (
            b"-9_223_372_036_854_775_809",
            Err(ErrorReason::IntegerTooLarge),
        ),
        (b"-0", Err(ErrorReason::NegativeZero)),
        (b"-00", Err(ErrorReason::ExpectedSingleZero)),
        (b"00", Err(ErrorReason::ExpectedSingleZero)),
        (b"1000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"_1", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"-1_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_0000", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_00", Err(ErrorReason::IncorrectDigitGrouping)),
        (b"1_000_0000", Err(ErrorReason::IncorrectDigitGrouping)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        match expected {
            Ok(expected_value) => {
                assert_eq!(decoder.consume_integer(), Ok(expected_value), "{msg}",)
            }
            Err(reason) => assert_eq!(decoder.consume_integer().expect_err(&msg).reason, reason),
        };
        if expected.is_ok() {
            decoder.close().expect(&msg);
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
        (b"\"\\\"", Err(ErrorReason::IncompleteEscapeSequence)),
        (b"\"\\0\"", Err(ErrorReason::IncompleteEscapeSequence)),
        (b"\"\\g0\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\0g\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\20\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\21\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\5b\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\5d\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\7e\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\80\"", Err(ErrorReason::InvalidEscapeSequence)),
        (b"\"\\ff\"", Err(ErrorReason::InvalidEscapeSequence)),
        (
            br#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#,
            Ok("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f".to_string()),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        match &expected {
            Ok(expected_value) => {
                assert_eq!(decoder.consume_string().as_ref(), Ok(expected_value), "{msg}", )
            }
            Err(reason) => assert_eq!(decoder.consume_string().expect_err(&msg).reason, *reason, "{msg}"),
        };
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_year() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedDateOrTime)),
        (b"T", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"!", Err(ErrorReason::ExpectedDateOrTime)),
        (b"D0", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D00", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D000", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D0000", Err(ErrorReason::YearOutOfRange)),
        (b"D0001", Ok(Year { y: 1u16 })),
        (b"D2024", Ok(Year { y: 2024 })),
        (b"D9999", Ok(Year { y: 9999 })),
        (b"D999a", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D10000", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D1000T", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024!", Err(ErrorReason::MalformedDateTimeTzOffset)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year();
        match expected.clone() {
            Ok(expected_value) => assert_eq!(result, Ok(expected_value), "{msg}"),
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_year_month() {
    for (bytes, expected) in [
        (b"D2024".as_slice(), Err(ErrorReason::ExpectedYearMonth)),
        (b"D2024Z", Err(ErrorReason::ExpectedYearMonth)),
        (b"D2024-W01", Err(ErrorReason::ExpectedYearMonth)),
        (b"D2024-", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-0", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-1", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-1x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-12x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-00", Err(ErrorReason::MonthOutOfRange)),
        (b"D2024-01", Ok(YearMonth { y: 2024, mo: 1 })),
        (b"D2024-12", Ok(YearMonth { y: 2024, mo: 12 })),
        (b"D2024-13", Err(ErrorReason::MonthOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month();
        match expected.clone() {
            Ok(expected_value) => assert_eq!(result, Ok(expected_value), "{msg}"),
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_year_week() {
    for (bytes, expected) in [
        (b"D2024".as_slice(), Err(ErrorReason::ExpectedYearWeek)),
        (b"D2024Z", Err(ErrorReason::ExpectedYearWeek)),
        (b"D2024-01", Err(ErrorReason::ExpectedYearWeek)),
        (b"D2024-", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W0", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W1", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W1x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W12x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W00", Err(ErrorReason::WeekOutOfRange)),
        (b"D2024-W01", Ok(YearWeek { y: 2024, w: 1 })),
        (b"D2024-W53", Ok(YearWeek { y: 2024, w: 53 })),
        (b"D2024-W54", Err(ErrorReason::WeekOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_week();
        match expected.clone() {
            Ok(expected_value) => assert_eq!(result, Ok(expected_value), "{msg}"),
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_year_month_day() {
    for (bytes, expected) in [
        (
            b"D2024-01".as_slice(),
            Err(ErrorReason::ExpectedYearMonthDay),
        ),
        (b"D2024-01Z", Err(ErrorReason::ExpectedYearMonthDay)),
        (b"D2024-W01-01", Err(ErrorReason::ExpectedYearMonthDay)),
        (b"D2024-01-", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-W01-", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-01-x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-01-0", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-01-1", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-01-1x", Err(ErrorReason::MalformedDateTimeTzOffset)),
        (b"D2024-01-12x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2024-01-00", Err(ErrorReason::DayOutOfRange)),
        (
            b"D2024-01-01",
            Ok(YearMonthDay {
                y: 2024,
                mo: 1,
                d: 1,
            }),
        ),
        (
            b"D2024-01-31",
            Ok(YearMonthDay {
                y: 2024,
                mo: 1,
                d: 31,
            }),
        ),
        (b"D2024-01-32", Err(ErrorReason::DayOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day();
        match expected.clone() {
            Ok(expected_value) => assert_eq!(result, Ok(expected_value), "{msg}"),
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
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
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_close_list().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
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
