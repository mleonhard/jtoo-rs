use jtoo::{
    escape_ascii, Decoder, ErrorReason, TzOffset, Year, YearMonth, YearMonthDay, YearMonthDayHour,
    YearMonthDayHourMinute, YearMonthDayHourMinuteMicrosecond, YearMonthDayHourMinuteMillisecond,
    YearMonthDayHourMinuteNanosecond, YearMonthDayHourMinuteSecond, YearMonthTzOffset,
    YearTzOffset, YearWeek, YearWeekDay, YearWeekDayHour, YearWeekTzOffset,
};

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
fn consume_integer() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedInteger)),
        (b"\"a\"", Err(ErrorReason::ExpectedInteger)),
        (b"T", Err(ErrorReason::ExpectedInteger)),
        (b"!", Err(ErrorReason::ExpectedInteger)),
        (b"-0", Err(ErrorReason::NegativeZero)),
        (b"0", Ok(0)),
        (b"1", Ok(1)),
        (b"12", Ok(12)),
        (b"123", Ok(123)),
        (b"1_234", Ok(1234)),
        (b"12_345", Ok(12345)),
        (b"123_456", Ok(123456)),
        (b"1_234_567", Ok(1234567)),
        (b"-1_234_567", Ok(-1234567)),
        (b"-123_456", Ok(-123456)),
        (b"-12_345", Ok(-12345)),
        (b"-1_234", Ok(-1234)),
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
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
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
fn consume_year() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedYear)),
        (b"T", Err(ErrorReason::ExpectedYear)),
        (b"!", Err(ErrorReason::ExpectedYear)),
        (b"D0", Err(ErrorReason::MalformedDate)),
        (b"D00", Err(ErrorReason::MalformedDate)),
        (b"D000", Err(ErrorReason::MalformedDate)),
        (b"D2029Z", Err(ErrorReason::ExpectedYear)),
        (b"D2029-08", Err(ErrorReason::ExpectedYear)),
        (b"D0000", Err(ErrorReason::YearOutOfRange)),
        (b"D0001", Ok(Year { y: 1u16 })),
        (b"D2029", Ok(Year { y: 2029 })),
        (b"D9999", Ok(Year { y: 9999 })),
        (b"D999a", Err(ErrorReason::MalformedDate)),
        (b"D10000", Err(ErrorReason::MalformedDate)),
        (b"D1000T", Err(ErrorReason::MalformedDate)),
        (b"D2029!", Err(ErrorReason::MalformedDate)),
        (b"D2029-01", Err(ErrorReason::ExpectedYear)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_tz_offset() {
    for (bytes, expected) in [
        (b"D2029".as_slice(), Err(ErrorReason::ExpectedTzOffset)),
        (b"D2029-01", Err(ErrorReason::ExpectedYearTzOffset)),
        // Hour part
        (b"D2029!", Err(ErrorReason::MalformedDate)),
        (b"D2029+", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+00", Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ)),
        (b"D2029~00", Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ)),
        (
            b"D2029+01",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: 1, m: 0 },
            }),
        ),
        (
            b"D2029~01",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: -1, m: 0 },
            }),
        ),
        (
            b"D2029+23",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: 23, m: 0 },
            }),
        ),
        (
            b"D2029~23",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: -23, m: 0 },
            }),
        ),
        (b"D2029+24", Err(ErrorReason::TimezoneOffsetHourOutOfRange)),
        // Minute part
        (b"D2029+08!", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~08!", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+08:", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~08:", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+08:0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~08:0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029+08:0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029~08:0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (
            b"D2029+00:00",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D2029~08:00",
            Err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted),
        ),
        (
            b"D2029+08:01",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: 8, m: 1 },
            }),
        ),
        (
            b"D2029~08:01",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: -8, m: 1 },
            }),
        ),
        (
            b"D2029+08:59",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: 8, m: 59 },
            }),
        ),
        (
            b"D2029~08:59",
            Ok(YearTzOffset {
                y: 2029,
                tz: TzOffset { h: -8, m: 59 },
            }),
        ),
        (
            b"D2029+08:60",
            Err(ErrorReason::TimezoneOffsetMinuteOutOfRange),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_tz_offset();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month() {
    for (bytes, expected) in [
        (b"D2029".as_slice(), Err(ErrorReason::ExpectedMonth)),
        (b"D2029Z", Err(ErrorReason::ExpectedMonth)),
        (b"D2029-W08", Err(ErrorReason::ExpectedMonth)),
        (b"D2029-", Err(ErrorReason::MalformedDate)),
        (b"D2029-0", Err(ErrorReason::MalformedDate)),
        (b"D2029-0x", Err(ErrorReason::MalformedDate)),
        (b"D2029-08x", Err(ErrorReason::MalformedDate)),
        (b"D2029-08Z", Err(ErrorReason::ExpectedYearMonth)),
        (b"D2029-08-01", Err(ErrorReason::ExpectedYearMonth)),
        (b"D2029-00", Err(ErrorReason::MonthOutOfRange)),
        (b"D2029-01", Ok(YearMonth { y: 2029, mo: 1 })),
        (b"D2029-12", Ok(YearMonth { y: 2029, mo: 12 })),
        (b"D2029-13", Err(ErrorReason::MonthOutOfRange)),
        (b"D2029-08-01", Err(ErrorReason::ExpectedYearMonth)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_tz_offset() {
    for (bytes, expected) in [
        (b"D2029-08".as_slice(), Err(ErrorReason::ExpectedTzOffset)),
        (
            b"D2029-08-01".as_slice(),
            Err(ErrorReason::ExpectedYearMonthTzOffset),
        ),
        (
            b"D2029-08-01Z".as_slice(),
            Err(ErrorReason::ExpectedYearMonthTzOffset),
        ),
        (b"D2029-W08", Err(ErrorReason::ExpectedMonth)),
        (b"D2029-08-07", Err(ErrorReason::ExpectedYearMonthTzOffset)),
        (b"D2029-08+", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-08+x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-08+0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-08+0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-08+01x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (
            b"D2029-08Z",
            Ok(YearMonthTzOffset {
                y: 2029,
                mo: 8,
                tz: TzOffset { h: 0, m: 0 },
            }),
        ),
        (
            b"D2029-08+01",
            Ok(YearMonthTzOffset {
                y: 2029,
                mo: 8,
                tz: TzOffset { h: 1, m: 0 },
            }),
        ),
        (
            b"D2029-08+07:01",
            Ok(YearMonthTzOffset {
                y: 2029,
                mo: 8,
                tz: TzOffset { h: 7, m: 1 },
            }),
        ),
        (
            b"D2029-08~07:06",
            Ok(YearMonthTzOffset {
                y: 2029,
                mo: 8,
                tz: TzOffset { h: -7, m: 6 },
            }),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_tz_offset();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_week() {
    for (bytes, expected) in [
        (b"D2029".as_slice(), Err(ErrorReason::ExpectedWeek)),
        (b"D2029Z", Err(ErrorReason::ExpectedWeek)),
        (b"D2029-01", Err(ErrorReason::ExpectedWeek)),
        (b"D2029-", Err(ErrorReason::MalformedDate)),
        (b"D2029-W", Err(ErrorReason::MalformedDate)),
        (b"D2029-W0", Err(ErrorReason::MalformedDate)),
        (b"D2029-W0x", Err(ErrorReason::MalformedDate)),
        (b"D2029-W01x", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-01", Err(ErrorReason::ExpectedYearWeek)),
        (b"D2029-W08Z", Err(ErrorReason::ExpectedYearWeek)),
        (b"D2029-W00", Err(ErrorReason::WeekOutOfRange)),
        (b"D2029-W01", Ok(YearWeek { y: 2029, w: 1 })),
        (b"D2029-W53", Ok(YearWeek { y: 2029, w: 53 })),
        (b"D2029-W54", Err(ErrorReason::WeekOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_week();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_week_tz_offset() {
    for (bytes, expected) in [
        (b"D2029-W08".as_slice(), Err(ErrorReason::ExpectedTzOffset)),
        (b"D2029-08", Err(ErrorReason::ExpectedWeek)),
        (b"D2029-W08-01", Err(ErrorReason::ExpectedYearWeekTzOffset)),
        (b"D2029-W08-01Z", Err(ErrorReason::ExpectedYearWeekTzOffset)),
        (b"D2029-W08+", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-W08+x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-W08+0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-W08+0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D2029-W08+01x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (
            b"D2029-W08Z",
            Ok(YearWeekTzOffset {
                y: 2029,
                w: 8,
                tz: TzOffset { h: 0, m: 0 },
            }),
        ),
        (
            b"D2029-W08+01",
            Ok(YearWeekTzOffset {
                y: 2029,
                w: 8,
                tz: TzOffset { h: 1, m: 0 },
            }),
        ),
        (
            b"D2029-W08+07:01",
            Ok(YearWeekTzOffset {
                y: 2029,
                w: 8,
                tz: TzOffset { h: 7, m: 1 },
            }),
        ),
        (
            b"D2029-W08~07:06",
            Ok(YearWeekTzOffset {
                y: 2029,
                w: 8,
                tz: TzOffset { h: -7, m: 6 },
            }),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_week_tz_offset();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

// TODO: Test remaining tzoffset methods.
// TODO: Test remaining time methods.
// TODO: Test has_another_list_item

#[test]
fn consume_year_month_day() {
    for (bytes, expected) in [
        (b"D2029-08".as_slice(), Err(ErrorReason::ExpectedDay)),
        (b"D2029-08-09Z", Err(ErrorReason::ExpectedYearMonthDay)),
        (b"D2029-W08-01", Err(ErrorReason::ExpectedMonth)),
        (b"D2029-08-09T00", Err(ErrorReason::ExpectedYearMonthDay)),
        (b"D2029-08-", Err(ErrorReason::MalformedDate)),
        (b"D2029-08-x", Err(ErrorReason::MalformedDate)),
        (b"D2029-08-0", Err(ErrorReason::MalformedDate)),
        (b"D2029-08-0x", Err(ErrorReason::MalformedDate)),
        (b"D2029-08-01x", Err(ErrorReason::MalformedDate)),
        (b"D2029-08-00", Err(ErrorReason::DayOutOfRange)),
        (
            b"D2029-08-01",
            Ok(YearMonthDay {
                y: 2029,
                mo: 8,
                d: 1,
            }),
        ),
        (
            b"D2029-08-31",
            Ok(YearMonthDay {
                y: 2029,
                mo: 8,
                d: 31,
            }),
        ),
        (b"D2029-08-32", Err(ErrorReason::DayOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_week_day() {
    for (bytes, expected) in [
        (b"D2029-08".as_slice(), Err(ErrorReason::ExpectedWeek)),
        (b"D2029-W08", Err(ErrorReason::ExpectedDay)),
        (b"D2029-W08-01Z", Err(ErrorReason::ExpectedYearWeekDay)),
        (b"D2029-08-01", Err(ErrorReason::ExpectedWeek)),
        (b"D2029-W08-", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-x", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-0", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-0x", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-01x", Err(ErrorReason::MalformedDate)),
        (b"D2029-W08-00", Err(ErrorReason::DayOutOfRange)),
        (
            b"D2029-W08-01",
            Ok(YearWeekDay {
                y: 2029,
                w: 8,
                d: 1,
            }),
        ),
        (
            b"D2029-W08-31",
            Ok(YearWeekDay {
                y: 2029,
                w: 8,
                d: 31,
            }),
        ),
        (b"D2029-W08-32", Err(ErrorReason::DayOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_week_day();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour() {
    for (bytes, expected) in [
        (b"D2029-08-07".as_slice(), Err(ErrorReason::ExpectedHour)),
        (b"D2029-W08-07T06", Err(ErrorReason::ExpectedMonth)),
        (
            b"D2029-08-07T06Z",
            Err(ErrorReason::ExpectedYearMonthDayHour),
        ),
        (
            b"D2029-08-07T06:05",
            Err(ErrorReason::ExpectedYearMonthDayHour),
        ),
        (b"D2029-08-07T", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07Tx", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T0", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T0x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T1", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T1x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T12x", Err(ErrorReason::MalformedTime)),
        (
            b"D2029-08-07T01",
            Ok(YearMonthDayHour {
                y: 2029,
                mo: 8,
                d: 7,
                h: 1,
            }),
        ),
        (
            b"D2029-08-07T23",
            Ok(YearMonthDayHour {
                y: 2029,
                mo: 8,
                d: 7,
                h: 23,
            }),
        ),
        (b"D2029-08-07T24", Err(ErrorReason::HourOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_week_day_hour() {
    for (bytes, expected) in [
        (b"D2029-W08-07".as_slice(), Err(ErrorReason::ExpectedHour)),
        (
            b"D2029-W08-07T06Z",
            Err(ErrorReason::ExpectedYearWeekDayHour),
        ),
        (b"D2029-08-07T06", Err(ErrorReason::ExpectedWeek)),
        (b"D2029-W08-07T", Err(ErrorReason::MalformedTime)),
        (b"D2029-W08-07Tx", Err(ErrorReason::MalformedTime)),
        (b"D2029-W08-07T0", Err(ErrorReason::MalformedTime)),
        (b"D2029-W08-07T0x", Err(ErrorReason::MalformedTime)),
        (b"D2029-W08-07T00x", Err(ErrorReason::MalformedTime)),
        (
            b"D2029-W08-07T00",
            Ok(YearWeekDayHour {
                y: 2029,
                w: 8,
                d: 7,
                h: 0,
            }),
        ),
        (
            b"D2029-W08-07T23",
            Ok(YearWeekDayHour {
                y: 2029,
                w: 8,
                d: 7,
                h: 23,
            }),
        ),
        (b"D2029-W08-07T24", Err(ErrorReason::HourOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_week_day_hour();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour_minute() {
    for (bytes, expected) in [
        (
            b"D2029-08-07T06".as_slice(),
            Err(ErrorReason::ExpectedMinute),
        ),
        (
            b"D2029-W08-07T06:05".as_slice(),
            Err(ErrorReason::ExpectedMonth),
        ),
        (
            b"D2029-08-07T06:05Z",
            Err(ErrorReason::ExpectedYearMonthDayHourMinute),
        ),
        (
            b"D2029-08-07T06:05:04",
            Err(ErrorReason::ExpectedYearMonthDayHourMinute),
        ),
        (b"D2029-08-07T06:", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:0", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:0x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:00x", Err(ErrorReason::MalformedTime)),
        (
            b"D2029-08-07T06:01",
            Ok(YearMonthDayHourMinute {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 1,
            }),
        ),
        (
            b"D2029-08-07T06:59",
            Ok(YearMonthDayHourMinute {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 59,
            }),
        ),
        (b"D2029-08-07T06:60", Err(ErrorReason::MinuteOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour_minute();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour_minute_second() {
    for (bytes, expected) in [
        (
            b"D2029-08-07T06:05".as_slice(),
            Err(ErrorReason::ExpectedSecond),
        ),
        (
            b"D2029-08-07T06:05:04Z",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteSecond),
        ),
        (b"D2029-W08-07T06:05:04", Err(ErrorReason::ExpectedMonth)),
        (
            b"D2029-08-07T06:05:04.000",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteSecond),
        ),
        (b"D2029-08-07T06:05:", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:0", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:0x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:00x", Err(ErrorReason::MalformedTime)),
        (
            b"D2029-08-07T06:05:01",
            Ok(YearMonthDayHourMinuteSecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                s: 1,
            }),
        ),
        (
            b"D2029-08-07T06:05:60",
            Ok(YearMonthDayHourMinuteSecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                s: 60,
            }),
        ),
        (b"D2029-08-07T06:05:61", Err(ErrorReason::SecondOutOfRange)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour_minute_second();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour_minute_millisecond() {
    for (bytes, expected) in [
        (
            b"D2029-08-07T06:05:04".as_slice(),
            Err(ErrorReason::ExpectedMillisecond),
        ),
        (
            b"D2029-08-07T06:05:04.003Z",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecond),
        ),
        (
            b"D2029-W08-07T06:05:04.003",
            Err(ErrorReason::ExpectedMonth),
        ),
        (
            b"D2029-08-07T06:05:04.003_002",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteMillisecond),
        ),
        (b"D2029-08-07T06:05:04.", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:04.x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:04.0", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:04.0x", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:04.00", Err(ErrorReason::MalformedTime)),
        (b"D2029-08-07T06:05:04.00x", Err(ErrorReason::MalformedTime)),
        (
            b"D2029-08-07T06:05:04.000x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.0000",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:00.000",
            Ok(YearMonthDayHourMinuteMillisecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ms: 0,
            }),
        ),
        (
            b"D2029-08-07T06:05:00.999",
            Ok(YearMonthDayHourMinuteMillisecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ms: 999,
            }),
        ),
        (
            b"D2029-08-07T06:05:04.321",
            Ok(YearMonthDayHourMinuteMillisecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ms: 4321,
            }),
        ),
        (
            b"D2029-08-07T06:05:60.999",
            Ok(YearMonthDayHourMinuteMillisecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ms: 60999,
            }),
        ),
        (
            b"D2029-08-07T06:05:61.000",
            Err(ErrorReason::SecondOutOfRange),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour_minute_millisecond();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour_minute_microsecond() {
    for (bytes, expected) in [
        (
            b"D2029-08-07T06:05:04.333".as_slice(),
            Err(ErrorReason::ExpectedMicrosecond),
        ),
        (
            b"D2029-08-07T06:05:04.333_222Z",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecond),
        ),
        (
            b"D2029-W08-07T06:05:04.333_222",
            Err(ErrorReason::ExpectedMonth),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_111",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteMicrosecond),
        ),
        (
            b"D2029-08-07T06:05:04.333_",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_0",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_0x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_00",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_00x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_000x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_0000",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:00.000_000",
            Ok(YearMonthDayHourMinuteMicrosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                us: 00_000_000,
            }),
        ),
        (
            b"D2029-08-07T06:05:00.000_999",
            Ok(YearMonthDayHourMinuteMicrosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                us: 00_000_999,
            }),
        ),
        (
            b"D2029-08-07T06:05:04.333_219",
            Ok(YearMonthDayHourMinuteMicrosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                us: 04_333_219,
            }),
        ),
        (
            b"D2029-08-07T06:05:60.999_999",
            Ok(YearMonthDayHourMinuteMicrosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                us: 60_999_999,
            }),
        ),
        (
            b"D2029-08-07T06:05:61.000_000",
            Err(ErrorReason::SecondOutOfRange),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour_minute_microsecond();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
        }
    }
}

#[test]
fn consume_year_month_day_hour_minute_nanosecond() {
    for (bytes, expected) in [
        (
            b"D2029-08-07T06:05:04.333_222".as_slice(),
            Err(ErrorReason::ExpectedNanosecond),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_111Z",
            Err(ErrorReason::ExpectedYearMonthDayHourMinuteNanosecond),
        ),
        (
            b"D2029-W08-07T06:05:04.333_222_111",
            Err(ErrorReason::ExpectedMonth),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_0",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_0x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_00",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_00x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_000x",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_0000",
            Err(ErrorReason::MalformedTime),
        ),
        (
            b"D2029-08-07T06:05:00.000_000_000",
            Ok(YearMonthDayHourMinuteNanosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ns: 00_000_000_000,
            }),
        ),
        (
            b"D2029-08-07T06:05:00.000_000_999",
            Ok(YearMonthDayHourMinuteNanosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ns: 00_000_000_999,
            }),
        ),
        (
            b"D2029-08-07T06:05:04.333_222_198",
            Ok(YearMonthDayHourMinuteNanosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ns: 04_333_222_198,
            }),
        ),
        (
            b"D2029-08-07T06:05:60.999_999_999",
            Ok(YearMonthDayHourMinuteNanosecond {
                y: 2029,
                mo: 8,
                d: 7,
                h: 6,
                m: 5,
                ns: 60_999_999_999,
            }),
        ),
        (
            b"D2029-08-07T06:05:61.000_000",
            Err(ErrorReason::SecondOutOfRange),
        ),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_year_month_day_hour_minute_nanosecond();
        match expected {
            Ok(expected_value) => {
                assert_eq!(result, Ok(expected_value), "{msg}");
                decoder.close().expect(&msg);
            }
            Err(expected_reason) => {
                let e = result.expect_err(&msg);
                assert_eq!(e.reason, expected_reason, "{msg}");
            }
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
fn list_not_a_list() {
    assert_eq!(
        Decoder::new(b"T").consume_open_list().unwrap_err().reason,
        ErrorReason::ExpectedList
    );
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
