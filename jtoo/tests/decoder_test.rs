use jtoo::{escape_ascii, Date, DateTimeTzOffset, Decoder, ErrorReason, Time, TzOffset};

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
fn consume_date_time_tz_offset() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(ErrorReason::ExpectedDateOrTime)),
        (b"Z", Err(ErrorReason::ExpectedDateOrTime)),
        (b"!", Err(ErrorReason::ExpectedDateOrTime)),
        // Year ///////////////////////////////////////////////////////////////////////////////////
        (b"D1", Err(ErrorReason::MalformedDate)),
        (b"D11", Err(ErrorReason::MalformedDate)),
        (b"D111", Err(ErrorReason::MalformedDate)),
        (b"D111x", Err(ErrorReason::MalformedDate)),
        (b"D0001", Ok(DateTimeTzOffset::Date(Date::Year { y: 1 }))),
        (b"D2029", Ok(DateTimeTzOffset::Date(Date::Year { y: 2029 }))),
        (b"D9999", Ok(DateTimeTzOffset::Date(Date::Year { y: 9999 }))),
        (b"D10000", Err(ErrorReason::MalformedDate)),
        (b"D9999x", Err(ErrorReason::MalformedDate)),
        (b"D9999T", Err(ErrorReason::MalformedDate)),
        // Year + TzOffset Hour ///////////////////////////////////////////////////////////////////
        (
            b"D9999Z",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (b"D9999+", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+00", Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ)),
        (b"D9999~00", Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ)),
        (
            b"D9999+01",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: 1, m: 0 },
            )),
        ),
        (
            b"D9999~01",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: -1, m: 0 },
            )),
        ),
        (
            b"D9999+23",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: 23, m: 0 },
            )),
        ),
        (
            b"D9999~23",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: -23, m: 0 },
            )),
        ),
        (b"D9999+24", Err(ErrorReason::TimezoneOffsetHourOutOfRange)),
        (b"D9999~24", Err(ErrorReason::TimezoneOffsetHourOutOfRange)),
        // Year + TzOffset Minute /////////////////////////////////////////////////////////////////
        (b"D9999+08x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~08x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+08T", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~08T", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+08:", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~08:", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+08:0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~08:0", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999+08:0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (b"D9999~08:0x", Err(ErrorReason::MalformedTimeZoneOffset)),
        (
            b"D9999+00:00",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D9999~00:00",
            Err(ErrorReason::ZeroTimeZoneOffsetShouldBeZ),
        ),
        (
            b"D9999+08:00",
            Err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted),
        ),
        (
            b"D9999~08:00",
            Err(ErrorReason::ZeroTimeZoneMinutesShouldBeOmitted),
        ),
        (
            b"D9999+08:01",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: 8, m: 1 },
            )),
        ),
        (
            b"D9999~08:01",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: -8, m: 1 },
            )),
        ),
        (
            b"D9999+08:59",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: 8, m: 59 },
            )),
        ),
        (
            b"D9999~08:59",
            Ok(DateTimeTzOffset::DateTz(
                Date::Year { y: 9999 },
                TzOffset { h: -8, m: 59 },
            )),
        ),
        (
            b"D9999+08:60",
            Err(ErrorReason::TimezoneOffsetMinuteOutOfRange),
        ),
        (
            b"D9999~08:60",
            Err(ErrorReason::TimezoneOffsetMinuteOutOfRange),
        ),
        // Month //////////////////////////////////////////////////////////////////////////////////
        (b"D9999-", Err(ErrorReason::MalformedDate)),
        (b"D9999-x", Err(ErrorReason::MalformedDate)),
        (b"D9999-T", Err(ErrorReason::MalformedDate)),
        (b"D9999-0", Err(ErrorReason::MalformedDate)),
        (b"D9999-0x", Err(ErrorReason::MalformedDate)),
        (b"D9999-0T", Err(ErrorReason::MalformedDate)),
        (b"D9999-08x", Err(ErrorReason::MalformedDate)),
        (b"D9999-00", Err(ErrorReason::MonthOutOfRange)),
        (
            b"D9999-01",
            Ok(DateTimeTzOffset::Date(Date::YearMonth { y: 9999, mo: 1 })),
        ),
        (
            b"D9999-12",
            Ok(DateTimeTzOffset::Date(Date::YearMonth { y: 9999, mo: 12 })),
        ),
        (b"D9999-13", Err(ErrorReason::MonthOutOfRange)),
        // Month + TzOffset ///////////////////////////////////////////////////////////////////////
        (
            b"D9999-08Z",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonth { y: 9999, mo: 8 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08+07",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonth { y: 9999, mo: 8 },
                TzOffset { h: 7, m: 0 },
            )),
        ),
        (
            b"D9999-08~07",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonth { y: 9999, mo: 8 },
                TzOffset { h: -7, m: 0 },
            )),
        ),
        (
            b"D9999-08+07:16",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonth { y: 9999, mo: 8 },
                TzOffset { h: 7, m: 16 },
            )),
        ),
        (
            b"D9999-08~07:16",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonth { y: 9999, mo: 8 },
                TzOffset { h: -7, m: 16 },
            )),
        ),
        // Week ///////////////////////////////////////////////////////////////////////////////////
        (b"D9999-", Err(ErrorReason::MalformedDate)),
        (b"D9999-W", Err(ErrorReason::MalformedDate)),
        (b"D9999-Wx", Err(ErrorReason::MalformedDate)),
        (b"D9999-WT", Err(ErrorReason::MalformedDate)),
        (b"D9999-W0", Err(ErrorReason::MalformedDate)),
        (b"D9999-W0x", Err(ErrorReason::MalformedDate)),
        (b"D9999-W0T", Err(ErrorReason::MalformedDate)),
        (b"D9999-W01x", Err(ErrorReason::MalformedDate)),
        (b"D9999-W01T", Err(ErrorReason::MalformedDate)),
        (b"D9999-W00", Err(ErrorReason::WeekOutOfRange)),
        (
            b"D9999-W01",
            Ok(DateTimeTzOffset::Date(Date::YearWeek { y: 9999, w: 1 })),
        ),
        (
            b"D9999-W53",
            Ok(DateTimeTzOffset::Date(Date::YearWeek { y: 9999, w: 53 })),
        ),
        (b"D9999-W54", Err(ErrorReason::WeekOutOfRange)),
        // Week + TzOffset ////////////////////////////////////////////////////////////////////////
        (
            b"D9999-W08Z",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearWeek { y: 9999, w: 8 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-W08+07",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearWeek { y: 9999, w: 8 },
                TzOffset { h: 7, m: 0 },
            )),
        ),
        (
            b"D9999-W08~07",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearWeek { y: 9999, w: 8 },
                TzOffset { h: -7, m: 0 },
            )),
        ),
        (
            b"D9999-W08+07:16",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearWeek { y: 9999, w: 8 },
                TzOffset { h: 7, m: 16 },
            )),
        ),
        (
            b"D9999-W08~07:16",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearWeek { y: 9999, w: 8 },
                TzOffset { h: -7, m: 16 },
            )),
        ),
        // Month + Day ////////////////////////////////////////////////////////////////////////////
        (b"D9999-08-", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-x", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-T", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-0", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-0x", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-0T", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-01x", Err(ErrorReason::MalformedDate)),
        (b"D9999-08-01T", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-00", Err(ErrorReason::DayOutOfRange)),
        (
            b"D9999-08-01",
            Ok(DateTimeTzOffset::Date(Date::YearMonthDay {
                y: 9999,
                mo: 8,
                d: 1,
            })),
        ),
        (
            b"D9999-08-31",
            Ok(DateTimeTzOffset::Date(Date::YearMonthDay {
                y: 9999,
                mo: 8,
                d: 31,
            })),
        ),
        (b"D9999-08-32", Err(ErrorReason::DayOutOfRange)),
        // Month + Day + TzOffset /////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07Z",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07+06",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                TzOffset { h: 6, m: 0 },
            )),
        ),
        (
            b"D9999-08-07~06",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                TzOffset { h: -6, m: 0 },
            )),
        ),
        (
            b"D9999-08-07+06:15",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                TzOffset { h: 6, m: 15 },
            )),
        ),
        (
            b"D9999-08-07~06:15",
            Ok(DateTimeTzOffset::DateTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                TzOffset { h: -6, m: 15 },
            )),
        ),
        // Week + Day /////////////////////////////////////////////////////////////////////////////
        (b"D9999-W08-", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-x", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-T", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-0", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-0x", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-0T", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-01x", Err(ErrorReason::MalformedDate)),
        (b"D9999-W08-01T", Err(ErrorReason::MalformedTime)),
        (b"D9999-W08-00", Err(ErrorReason::DayOutOfRange)),
        (
            b"D9999-W08-01",
            Ok(DateTimeTzOffset::Date(Date::YearWeekDay {
                y: 9999,
                w: 8,
                d: 1,
            })),
        ),
        (
            b"D9999-W08-31",
            Ok(DateTimeTzOffset::Date(Date::YearWeekDay {
                y: 9999,
                w: 8,
                d: 31,
            })),
        ),
        (b"D9999-W08-32", Err(ErrorReason::DayOutOfRange)),
        // Hour ///////////////////////////////////////////////////////////////////////////////////
        (b"D9999-08-07T", Err(ErrorReason::MalformedTime)),
        (b"T", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07Tx", Err(ErrorReason::MalformedTime)),
        (b"Tx", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07TZ", Err(ErrorReason::MalformedTime)),
        (b"TZ", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T0", Err(ErrorReason::MalformedTime)),
        (b"T0", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T0x", Err(ErrorReason::MalformedTime)),
        (b"T0x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T0Z", Err(ErrorReason::MalformedTime)),
        (b"T0Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T12x", Err(ErrorReason::MalformedTime)),
        (b"T12x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T01",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 1 },
            )),
        ),
        (b"T01", Ok(DateTimeTzOffset::Time(Time::Hour { h: 1 }))),
        (
            b"D9999-08-07T23",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 23 },
            )),
        ),
        (b"T23", Ok(DateTimeTzOffset::Time(Time::Hour { h: 23 }))),
        (b"D9999-08-07T24", Err(ErrorReason::HourOutOfRange)),
        (b"T24", Err(ErrorReason::HourOutOfRange)),
        // Hour + TzOffset ////////////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 6 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::Hour { h: 6 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06+05",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 6 },
                TzOffset { h: 5, m: 0 },
            )),
        ),
        (
            b"T06+05",
            Ok(DateTimeTzOffset::TimeTz(
                Time::Hour { h: 6 },
                TzOffset { h: 5, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06~05",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 6 },
                TzOffset { h: -5, m: 0 },
            )),
        ),
        (
            b"T06~05",
            Ok(DateTimeTzOffset::TimeTz(
                Time::Hour { h: 6 },
                TzOffset { h: -5, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06+05:14",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 6 },
                TzOffset { h: 5, m: 14 },
            )),
        ),
        (
            b"T06+05:14",
            Ok(DateTimeTzOffset::TimeTz(
                Time::Hour { h: 6 },
                TzOffset { h: 5, m: 14 },
            )),
        ),
        (
            b"D9999-08-07T06~05:14",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::Hour { h: 6 },
                TzOffset { h: -5, m: 14 },
            )),
        ),
        (
            b"T06~05:14",
            Ok(DateTimeTzOffset::TimeTz(
                Time::Hour { h: 6 },
                TzOffset { h: -5, m: 14 },
            )),
        ),
        // Minute /////////////////////////////////////////////////////////////////////////////////
        (b"D9999-08-07T06:", Err(ErrorReason::MalformedTime)),
        (b"T06:", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:x", Err(ErrorReason::MalformedTime)),
        (b"T06:x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:Z", Err(ErrorReason::MalformedTime)),
        (b"T06:Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:0", Err(ErrorReason::MalformedTime)),
        (b"T06:0", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:0x", Err(ErrorReason::MalformedTime)),
        (b"T06:0x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:0Z", Err(ErrorReason::MalformedTime)),
        (b"T06:0Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:00x", Err(ErrorReason::MalformedTime)),
        (b"T06:00x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:01",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 1 },
            )),
        ),
        (
            b"T06:01",
            Ok(DateTimeTzOffset::Time(Time::HourMinute { h: 6, m: 1 })),
        ),
        (
            b"D9999-08-07T06:59",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 59 },
            )),
        ),
        (
            b"T06:59",
            Ok(DateTimeTzOffset::Time(Time::HourMinute { h: 6, m: 59 })),
        ),
        (b"D9999-08-07T06:60", Err(ErrorReason::MinuteOutOfRange)),
        (b"T06:60", Err(ErrorReason::MinuteOutOfRange)),
        // Minute + TzOffset //////////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06:05Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05+04",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 4, m: 0 },
            )),
        ),
        (
            b"T06:05+04",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 4, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05~04",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: -4, m: 0 },
            )),
        ),
        (
            b"T06:05~04",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: -4, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05+04:13",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 4, m: 13 },
            )),
        ),
        (
            b"T06:05+04:13",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: 4, m: 13 },
            )),
        ),
        (
            b"D9999-08-07T06:05~04:13",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: -4, m: 13 },
            )),
        ),
        (
            b"T06:05~04:13",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinute { h: 6, m: 5 },
                TzOffset { h: -4, m: 13 },
            )),
        ),
        // Second /////////////////////////////////////////////////////////////////////////////////
        (b"D9999-08-07T06:05:", Err(ErrorReason::MalformedTime)),
        (b"T06:05:", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:Z", Err(ErrorReason::MalformedTime)),
        (b"T06:05:Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:0", Err(ErrorReason::MalformedTime)),
        (b"T06:05:0", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:0x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:0x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:0Z", Err(ErrorReason::MalformedTime)),
        (b"T06:05:0Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:00x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:00x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:00",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 0 },
            )),
        ),
        (
            b"T06:05:00",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteSecond {
                h: 6,
                m: 5,
                s: 0,
            })),
        ),
        (
            b"D9999-08-07T06:05:60",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 60 },
            )),
        ),
        (
            b"T06:05:60",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteSecond {
                h: 6,
                m: 5,
                s: 60,
            })),
        ),
        (b"D9999-08-07T06:05:61", Err(ErrorReason::SecondOutOfRange)),
        (b"T06:05:61", Err(ErrorReason::SecondOutOfRange)),
        // Second + TzOffset //////////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06:05:04Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04+03",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 3, m: 0 },
            )),
        ),
        (
            b"T06:05:04+03",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 3, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04~03",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: -3, m: 0 },
            )),
        ),
        (
            b"T06:05:04~03",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: -3, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04+03:12",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 3, m: 12 },
            )),
        ),
        (
            b"T06:05:04+03:12",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: 3, m: 12 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04~03:12",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: -3, m: 12 },
            )),
        ),
        (
            b"T06:05:04~03:12",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteSecond { h: 6, m: 5, s: 4 },
                TzOffset { h: -3, m: 12 },
            )),
        ),
        // Millisecond ////////////////////////////////////////////////////////////////////////////
        (b"D9999-08-07T06:05:04.", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.Z", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.0", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.0", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.0x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.0x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.0Z", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.0Z", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.00", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.00", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.00x", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.00x", Err(ErrorReason::MalformedTime)),
        (b"D9999-08-07T06:05:04.00Z", Err(ErrorReason::MalformedTime)),
        (b"T06:05:04.00Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.000x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.000x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.0000",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.0000", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:00.000",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond { h: 6, m: 5, ms: 0 },
            )),
        ),
        (
            b"T06:05:00.000",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMillisecond {
                h: 6,
                m: 5,
                ms: 0,
            })),
        ),
        (
            b"D9999-08-07T06:05:00.999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 999,
                },
            )),
        ),
        (
            b"T06:05:00.999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMillisecond {
                h: 6,
                m: 5,
                ms: 999,
            })),
        ),
        (
            b"D9999-08-07T06:05:04.321",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4321,
                },
            )),
        ),
        (
            b"T06:05:04.321",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMillisecond {
                h: 6,
                m: 5,
                ms: 4321,
            })),
        ),
        (
            b"D9999-08-07T06:05:60.999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 60999,
                },
            )),
        ),
        (
            b"T06:05:60.999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMillisecond {
                h: 6,
                m: 5,
                ms: 60999,
            })),
        ),
        (
            b"D9999-08-07T06:05:61.000",
            Err(ErrorReason::SecondOutOfRange),
        ),
        (b"T06:05:61.000", Err(ErrorReason::SecondOutOfRange)),
        // Millisecond + TzOffset /////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04.333Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333+02",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 2, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333+02",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 2, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333~02",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: -2, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333~02",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: -2, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333+02:19",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 2, m: 19 },
            )),
        ),
        (
            b"T06:05:04.333+02:19",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: 2, m: 19 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333~02:19",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: -2, m: 19 },
            )),
        ),
        (
            b"T06:05:04.333~02:19",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMillisecond {
                    h: 6,
                    m: 5,
                    ms: 4333,
                },
                TzOffset { h: -2, m: 19 },
            )),
        ),
        // Microsecond ////////////////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04.333_",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_0",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_0", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_0x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_0x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_0Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_0Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_00",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_00", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_00x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_00x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_00Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_00Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_000x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_000x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_0000",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_0000", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:00.000_000",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond { h: 6, m: 5, us: 0 },
            )),
        ),
        (
            b"T06:05:00.000_000",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMicrosecond {
                h: 6,
                m: 5,
                us: 0,
            })),
        ),
        (
            b"D9999-08-07T06:05:00.000_999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 00_000_999,
                },
            )),
        ),
        (
            b"T06:05:00.000_999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMicrosecond {
                h: 6,
                m: 5,
                us: 00_000_999,
            })),
        ),
        (
            b"D9999-08-07T06:05:04.333_219",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_219,
                },
            )),
        ),
        (
            b"T06:05:04.333_219",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMicrosecond {
                h: 6,
                m: 5,
                us: 04_333_219,
            })),
        ),
        (
            b"D9999-08-07T06:05:60.999_999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 60_999_999,
                },
            )),
        ),
        (
            b"T06:05:60.999_999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteMicrosecond {
                h: 6,
                m: 5,
                us: 60_999_999,
            })),
        ),
        (
            b"D9999-08-07T06:05:61.000_000",
            Err(ErrorReason::SecondOutOfRange),
        ),
        (b"T06:05:61.000_000", Err(ErrorReason::SecondOutOfRange)),
        // Microsecond + TzOffset /////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04.333_222Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222+01",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 1, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222+01",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 1, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222~01",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: -1, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222~01",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: -1, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222+01:29",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 1, m: 29 },
            )),
        ),
        (
            b"T06:05:04.333_222+01:29",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: 1, m: 29 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222~01:29",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: -1, m: 29 },
            )),
        ),
        (
            b"T06:05:04.333_222~01:29",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteMicrosecond {
                    h: 6,
                    m: 5,
                    us: 04_333_222,
                },
                TzOffset { h: -1, m: 29 },
            )),
        ),
        // Nanosecond /////////////////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04.333_222_",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_0",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_0", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_0x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_0x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_0Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_0Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_00",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_00", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_00x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_00x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_00Z",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_00Z", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_000x",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_000x", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:04.333_222_0000",
            Err(ErrorReason::MalformedTime),
        ),
        (b"T06:05:04.333_222_0000", Err(ErrorReason::MalformedTime)),
        (
            b"D9999-08-07T06:05:00.000_000_000",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 00_000_000_000,
                },
            )),
        ),
        (
            b"T06:05:00.000_000_000",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteNanosecond {
                h: 6,
                m: 5,
                ns: 00_000_000_000,
            })),
        ),
        (
            b"D9999-08-07T06:05:00.000_000_999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 00_000_000_999,
                },
            )),
        ),
        (
            b"T06:05:00.000_000_999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteNanosecond {
                h: 6,
                m: 5,
                ns: 00_000_000_999,
            })),
        ),
        (
            b"D9999-08-07T06:05:04.333_222_198",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_198,
                },
            )),
        ),
        (
            b"T06:05:04.333_222_198",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteNanosecond {
                h: 6,
                m: 5,
                ns: 04_333_222_198,
            })),
        ),
        (
            b"D9999-08-07T06:05:60.999_999_999",
            Ok(DateTimeTzOffset::DateTime(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 60_999_999_999,
                },
            )),
        ),
        (
            b"T06:05:60.999_999_999",
            Ok(DateTimeTzOffset::Time(Time::HourMinuteNanosecond {
                h: 6,
                m: 5,
                ns: 60_999_999_999,
            })),
        ),
        (
            b"D9999-08-07T06:05:61.000_000_000",
            Err(ErrorReason::SecondOutOfRange),
        ),
        (b"T06:05:61.000_000_000", Err(ErrorReason::SecondOutOfRange)),
        // Nanosecond + TzOffset///////////////////////////////////////////////////////////////////
        (
            b"D9999-08-07T06:05:04.333_222_111Z",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222_111Z",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 0, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222_111+09",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 9, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222_111+09",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 9, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222_111~09",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: -9, m: 0 },
            )),
        ),
        (
            b"T06:05:04.333_222_111~09",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: -9, m: 0 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222_111+09:18",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 9, m: 18 },
            )),
        ),
        (
            b"T06:05:04.333_222_111+09:18",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: 9, m: 18 },
            )),
        ),
        (
            b"D9999-08-07T06:05:04.333_222_111~09:18",
            Ok(DateTimeTzOffset::DateTimeTz(
                Date::YearMonthDay {
                    y: 9999,
                    mo: 8,
                    d: 7,
                },
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: -9, m: 18 },
            )),
        ),
        (
            b"T06:05:04.333_222_111~09:18",
            Ok(DateTimeTzOffset::TimeTz(
                Time::HourMinuteNanosecond {
                    h: 6,
                    m: 5,
                    ns: 04_333_222_111,
                },
                TzOffset { h: -9, m: 18 },
            )),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        let result = decoder.consume_date_time_tz_offset();
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
fn date_time_tz_offset() {
    let date = Date::YearMonthDay {
        y: 9999,
        mo: 8,
        d: 7,
    };
    let time = Time::Hour { h: 6 };
    let tz_offset = TzOffset { h: 0, m: 0 };
    for (bytes, expected_date, expected_time, expected_tz_offset) in [
        (b"D9999-08-07".as_slice(), Some(&date), None, None),
        (b"D9999-08-07Z", Some(&date), None, Some(&tz_offset)),
        (b"D9999-08-07T06", Some(&date), Some(&time), None),
        (
            b"D9999-08-07T06Z",
            Some(&date),
            Some(&time),
            Some(&tz_offset),
        ),
        (b"T06", None, Some(&time), None),
        (b"T06Z", None, Some(&time), Some(&tz_offset)),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let value = Decoder::new(bytes)
            .consume_date_time_tz_offset()
            .expect(&msg);
        assert_eq!(value.date(), expected_date, "{msg}");
        assert_eq!(value.time(), expected_time, "{msg}");
        assert_eq!(value.tz_offset(), expected_tz_offset, "{msg}");
    }
}

#[test]
fn date() {
    for (bytes, y, mo, w, d) in [
        (b"D9999".as_slice(), Some(9999), None, None, None),
        (b"D9999-08".as_slice(), Some(9999), Some(8), None, None),
        (b"D9999-W08".as_slice(), Some(9999), None, Some(8), None),
        (
            b"D9999-08-07".as_slice(),
            Some(9999),
            Some(8),
            None,
            Some(7),
        ),
        (
            b"D9999-W08-07".as_slice(),
            Some(9999),
            None,
            Some(8),
            Some(7),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let date = *Decoder::new(bytes)
            .consume_date_time_tz_offset()
            .expect(&msg)
            .date()
            .expect(&msg);
        assert_eq!(date.year(), y, "{msg}");
        assert_eq!(date.month(), mo, "{msg}");
        assert_eq!(date.week(), w, "{msg}");
        assert_eq!(date.day(), d, "{msg}");
    }
}

#[test]
fn time() {
    for (bytes, h, m, s, ms, us, ns) in [
        (b"T21".as_slice(), Some(21), None, None, None, None, None),
        (b"T21:39", Some(21), Some(39), None, None, None, None),
        (b"T21:39:12", Some(21), Some(39), Some(12), None, None, None),
        (
            b"T21:39:12.345",
            Some(21),
            Some(39),
            Some(12),
            Some(12_345),
            None,
            None,
        ),
        (
            b"T21:39:12.345_678",
            Some(21),
            Some(39),
            Some(12),
            Some(12_345),
            Some(12_345_678),
            None,
        ),
        (
            b"T21:39:12.345_678_910",
            Some(21),
            Some(39),
            Some(12),
            Some(12_345),
            Some(12_345_678),
            Some(12_345_678_910),
        ),
    ] {
        let msg = format!("bytes=b\"{}\"", escape_ascii(bytes));
        let time = *Decoder::new(bytes)
            .consume_date_time_tz_offset()
            .expect(&msg)
            .time()
            .expect(&msg);
        assert_eq!(time.hour(), h, "{msg}");
        assert_eq!(time.minute(), m, "{msg}");
        assert_eq!(time.second(), s, "{msg}");
        assert_eq!(time.millisecond(), ms, "{msg}");
        assert_eq!(time.microsecond(), us, "{msg}");
        assert_eq!(time.nanosecond(), ns, "{msg}");
    }
}

// TODO: Test has_another_list_item

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

#[test]
fn debug() {
    let _ = format!("{:?}", Decoder::new(b"x"));
}
