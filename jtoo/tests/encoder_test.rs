use jtoo::{EncodeError, Encoder};

#[test]
fn empty() {
    let encoder = Encoder::new();
    assert_eq!(encoder.into_string(), Ok(String::new()));
}

#[test]
fn bool_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(false), Err(EncodeError::UnclosedString));
}

#[test]
fn bool_value() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.append_bool(false).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[T,F]".to_string()));
}

#[test]
fn byte_string_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_byte_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn byte_string_empty() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(encoder.into_string(), Ok("B".to_string()));
}

#[test]
fn byte_string_value() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    encoder
        .append_byte_string(&[
            0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2,
            0xe1, 0xf0,
        ])
        .unwrap();
    encoder.close_byte_string().unwrap();
    assert_eq!(
        encoder.into_string(),
        Ok("B0f1e2d3c4b5a69788796a5b4c3d2e1f0".to_string())
    );
}

#[test]
fn byte_string_closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_byte_string().unwrap();
    encoder.close_byte_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[B,T]".to_string()));
}

#[test]
fn byte_string_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    assert_eq!(
        encoder.append_bool(true),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedByteString));
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedByteString));
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
fn decimal_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(
        encoder.append_decimal(0, 0),
        Err(EncodeError::UnclosedString)
    );
}

#[test]
fn decimal() {
    for (value, exp, expected) in [
        // Zeros added before decimal point.
        (0, 0, "0.0"),
        (0, 1, "0.0"),
        (0, 2, "0.0"),
        (0, 3, "0.0"),
        (0, 4, "0.0"),
        (1, 1, "10.0"),
        (1, 2, "100.0"),
        (1, 3, "1_000.0"),
        (1, 4, "10_000.0"),
        (1, 5, "100_000.0"),
        (1, 6, "1_000_000.0"),
        (1, 7, "10_000_000.0"),
        (-1, 7, "-10_000_000.0"),
        (12, 7, "120_000_000.0"),
        (123, 7, "1_230_000_000.0"),
        (1234, 7, "12_340_000_000.0"),
        (-1234, 7, "-12_340_000_000.0"),
        // Exponent is zero.
        (1, 0, "1.0"),
        (-1, 0, "-1.0"),
        (12, 0, "12.0"),
        (-12, 0, "-12.0"),
        (123, 0, "123.0"),
        (-123, 0, "-123.0"),
        (1_234, 0, "1_234.0"),
        (-1_234, 0, "-1_234.0"),
        (12_345, 0, "12_345.0"),
        (-12_345, 0, "-12_345.0"),
        (123_456, 0, "123_456.0"),
        (-123_456, 0, "-123_456.0"),
        (1_234_567, 0, "1_234_567.0"),
        (-1_234_567, 0, "-1_234_567.0"),
        (i64::MAX, 0, "9_223_372_036_854_775_807.0"),
        (i64::MIN, 0, "-9_223_372_036_854_775_808.0"),
        // Zeros added after the decimal point.
        (0, -2, "0.00"),
        (0, -1, "0.0"),
        (1, -2, "0.01"),
        (-1, -2, "-0.01"),
        (10, -2, "0.10"),
        (1, -3, "0.001"),
        (1, -4, "0.000_1"),
        (10, -4, "0.001_0"),
        (100, -4, "0.010_0"),
        (1_000, -4, "0.100_0"),
        (10_000, -4, "1.000_0"),
        (1, -5, "0.000_01"),
        (1, -6, "0.000_001"),
        (1, -7, "0.000_000_1"),
        (-1, -7, "-0.000_000_1"),
        (1_234, -7, "0.000_123_4"),
        (-1_234, -7, "-0.000_123_4"),
        // No zeros added after decimal.
        (1, -1, "0.1"),
        (-1, -1, "-0.1"),
        (12, -2, "0.12"),
        (123, -3, "0.123"),
        (1_234, -4, "0.123_4"),
        (12_345, -5, "0.123_45"),
        (123_456, -6, "0.123_456"),
        (1_234_567, -7, "0.123_456_7"),
        (-1_234_567, -7, "-0.123_456_7"),
        (i64::MAX, -19, "0.922_337_203_685_477_580_7"),
        (i64::MIN, -19, "-0.922_337_203_685_477_580_8"),
        // Digits on both sides of decimal point.
        (10, -1, "1.0"),
        (12, -1, "1.2"),
        (123, -1, "12.3"),
        (1_234, -1, "123.4"),
        (12_345, -1, "1_234.5"),
        (10, -2, "0.10"),
        (123_456, -2, "1_234.56"),
        (1_234_567, -3, "1_234.567"),
        (12_345_678, -4, "1_234.567_8"),
        (12_345_678, -5, "123.456_78"),
        (12_345_678, -6, "12.345_678"),
        (12_345_678, -7, "1.234_567_8"),
        (i64::MAX, -18, "9.223_372_036_854_775_807"),
        (i64::MIN, -18, "-9.223_372_036_854_775_808"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_decimal(value, exp).unwrap();
        assert_eq!(
            encoder.into_string(),
            Ok(expected.to_string()),
            "value={value} exp={exp}"
        );
    }
}

#[test]
fn integer_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_integer(1), Err(EncodeError::UnclosedString));
}

#[test]
fn integer() {
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
        (i64::MAX, "9_223_372_036_854_775_807"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_integer(value).unwrap();
        assert_eq!(encoder.into_string(), Ok(expected.to_string()));
    }
}

#[test]
fn list_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_list(), Err(EncodeError::UnclosedString));
}

#[test]
fn list_empty() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[]".to_string()));
}

#[test]
fn list_nested() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[[]]".to_string()));
}

#[test]
fn list_items() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.open_byte_string().unwrap();
    encoder.append_byte_string(&[0xa1]).unwrap();
    encoder.close_byte_string().unwrap();
    encoder.open_list().unwrap();
    encoder.append_bool(false).unwrap();
    encoder.close_list().unwrap();
    encoder.open_string().unwrap();
    encoder.append_string("string1").unwrap();
    encoder.close_string().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(
        encoder.into_string(),
        Ok("[T,Ba1,[F],\"string1\"]".to_string())
    );
}

#[test]
fn list_closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[[],T]".to_string()));
}

#[test]
fn list_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedList));
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedList));
}

#[test]
fn not_in_list() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));
}

#[test]
fn not_in_list_nested() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));
}

#[test]
fn string_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn string_empty() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.into_string(), Ok("\"\"".to_string()));
}

#[test]
fn string_ok() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_string("string1").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.into_string(), Ok("\"string1\"".to_string()));
}

#[test]
fn string_ok2() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_string("a").unwrap();
    encoder.append_string("b").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.into_string(), Ok("\"ab\"".to_string()));
}

#[test]
fn string_escaped() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_string("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.into_string(), Ok(r#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#.to_string()));
}

#[test]
fn string_closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    encoder.close_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.into_string(), Ok("[\"\",T]".to_string()));
}

#[test]
fn string_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(true), Err(EncodeError::UnclosedString));
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedString));
    assert_eq!(encoder.into_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn close_string() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.close_string(), Err(EncodeError::NotInString));
}

#[test]
fn not_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.append_string("x"), Err(EncodeError::NotInString));
}

#[test]
fn timestamp_seconds() {
    for (value, expected) in [
        (0, Ok("S0")),
        (1, Ok("S1")),
        (10, Ok("S10")),
        (100, Ok("S100")),
        (1_000, Ok("S1_000")),
        (10_000, Ok("S10_000")),
        (100_000, Ok("S100_000")),
        (1_000_000, Ok("S1_000_000")),
        (1_234_567_890, Ok("S1_234_567_890")),
        (i64::MAX as u64, Ok("S9_223_372_036_854_775_807")),
        (i64::MAX as u64 + 1, Err(EncodeError::InvalidTimestamp)),
        (u64::MAX, Err(EncodeError::InvalidTimestamp)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_timestamp_seconds(value).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_seconds(value), Err(e));
            }
        }
    }
}

#[test]
fn timestamp_milliseconds() {
    for (value, expected) in [
        (0, Ok("S0.000")),
        (1, Ok("S0.001")),
        (10, Ok("S0.010")),
        (100, Ok("S0.100")),
        (1_000, Ok("S1.000")),
        (10_000, Ok("S10.000")),
        (100_000, Ok("S100.000")),
        (1_000_000, Ok("S1_000.000")),
        (1_234_567_890, Ok("S1_234_567.890")),
        (i64::MAX as u64, Ok("S9_223_372_036_854_775.807")),
        (i64::MAX as u64 + 1, Err(EncodeError::InvalidTimestamp)),
        (u64::MAX, Err(EncodeError::InvalidTimestamp)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_timestamp_milliseconds(value).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_milliseconds(value), Err(e));
            }
        }
    }
}

#[test]
fn timestamp_microseconds() {
    for (value, expected) in [
        (0, Ok("S0.000_000")),
        (1, Ok("S0.000_001")),
        (10, Ok("S0.000_010")),
        (100, Ok("S0.000_100")),
        (1_000, Ok("S0.001_000")),
        (10_000, Ok("S0.010_000")),
        (100_000, Ok("S0.100_000")),
        (1_000_000, Ok("S1.000_000")),
        (1_234_567_890, Ok("S1_234.567_890")),
        (i64::MAX as u64, Ok("S9_223_372_036_854.775_807")),
        (i64::MAX as u64 + 1, Err(EncodeError::InvalidTimestamp)),
        (u64::MAX, Err(EncodeError::InvalidTimestamp)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_timestamp_microseconds(value).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_microseconds(value), Err(e));
            }
        }
    }
}

#[test]
fn timestamp_nanoseconds() {
    for (value, expected) in [
        (0, Ok("S0.000_000_000")),
        (1, Ok("S0.000_000_001")),
        (10, Ok("S0.000_000_010")),
        (100, Ok("S0.000_000_100")),
        (1_000, Ok("S0.000_001_000")),
        (10_000, Ok("S0.000_010_000")),
        (100_000, Ok("S0.000_100_000")),
        (1_000_000, Ok("S0.001_000_000")),
        (1_234_567_890, Ok("S1.234_567_890")),
        (i64::MAX as u64, Ok("S9_223_372_036.854_775_807")),
        (i64::MAX as u64 + 1, Err(EncodeError::InvalidTimestamp)),
        (u64::MAX, Err(EncodeError::InvalidTimestamp)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_timestamp_nanosecond(value).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_nanosecond(value), Err(e));
            }
        }
    }
}

#[test]
fn year() {
    for (year, expected) in [
        (0, Err(EncodeError::InvalidYear)),
        (1, Ok("D1")),
        (1000, Ok("D1000")),
        (2024, Ok("D2024")),
        (9999, Ok("D9999")),
        (10000, Err(EncodeError::InvalidYear)),
        (u16::MAX, Err(EncodeError::InvalidYear)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_year(year).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(encoder.append_year(year).err(), Some(e));
            }
        }
    }
}

#[test]
fn year_week() {
    for (week, expected) in [
        (0, Err(EncodeError::InvalidWeek)),
        (1, Ok("D2021-W01")),
        (53, Ok("D2021-W53")),
        (54, Err(EncodeError::InvalidWeek)),
        (u8::MAX, Err(EncodeError::InvalidWeek)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_year(2021).unwrap();
        match expected {
            Ok(s) => {
                appender.append_week(week).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(appender.append_week(week).err(), Some(e));
            }
        }
    }
}

#[test]
fn year_week_weekday() {
    for (weekday, expected) in [
        (0, Err(EncodeError::InvalidWeekday)),
        (1, Ok("D2021-W02-01")),
        (7, Ok("D2021-W02-07")),
        (8, Err(EncodeError::InvalidWeekday)),
        (u8::MAX, Err(EncodeError::InvalidWeekday)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_year(2021).unwrap().append_week(2).unwrap();
        match expected {
            Ok(s) => {
                appender
                    .append_weekday(weekday)
                    .unwrap_or_else(|_| panic!("weekday={weekday}"));
                assert_eq!(
                    encoder.into_string(),
                    Ok(s.to_string()),
                    "weekday={weekday}"
                );
            }
            Err(e) => {
                assert_eq!(
                    appender.append_weekday(weekday).err(),
                    Some(e),
                    "weekday={weekday}"
                );
            }
        }
    }
}

#[test]
fn year_month() {
    for (month, expected) in [
        (0, Err(EncodeError::InvalidMonth)),
        (1, Ok("D2024-01")),
        (12, Ok("D2024-12")),
        (13, Err(EncodeError::InvalidMonth)),
        (u8::MAX, Err(EncodeError::InvalidMonth)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_year(2024).unwrap();
        match expected {
            Ok(s) => {
                appender.append_month(month).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(appender.append_month(month).err(), Some(e));
            }
        }
    }
}

#[test]
fn year_month_day() {
    for (month, day, expected) in [
        (1, 0, Err(EncodeError::InvalidDay)),
        (1, 1, Ok("D2024-01-01")),
        (2, 30, Ok("D2024-02-30")),
        (12, 31, Ok("D2024-12-31")),
        (1, 32, Err(EncodeError::InvalidDay)),
        (1, u8::MAX, Err(EncodeError::InvalidDay)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_year(2024).unwrap();
        match expected {
            Ok(s) => {
                appender
                    .append_month(month)
                    .unwrap()
                    .append_day(day)
                    .unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()));
            }
            Err(e) => {
                assert_eq!(
                    appender.append_month(month).unwrap().append_day(day).err(),
                    Some(e)
                );
            }
        }
    }
}

#[test]
fn year_month_day_hour() {
    for (hour, expected) in [
        (0, Ok("D2021-02-03T00")),
        (1, Ok("D2021-02-03T01")),
        (23, Ok("D2021-02-03T23")),
        (24, Err(EncodeError::InvalidHour)),
        (u8::MAX, Err(EncodeError::InvalidHour)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder
            .append_year(2021)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_day(3)
            .unwrap();
        match expected {
            Ok(s) => {
                appender
                    .append_hour(hour)
                    .unwrap_or_else(|_| panic!("hour={hour}"));
                assert_eq!(encoder.into_string(), Ok(s.to_string()), "hour={hour}");
            }
            Err(e) => {
                assert_eq!(appender.append_hour(hour).err(), Some(e), "hour={hour}");
            }
        }
    }
}

#[test]
fn hour() {
    for (hour, expected) in [
        (0, Ok("T00")),
        (1, Ok("T01")),
        (23, Ok("T23")),
        (24, Err(EncodeError::InvalidHour)),
        (u8::MAX, Err(EncodeError::InvalidHour)),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder.append_hour(hour).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()), "hour={hour}");
            }
            Err(e) => {
                assert_eq!(encoder.append_hour(hour).err(), Some(e), "hour={hour}");
            }
        }
    }
}

#[test]
fn hour_minute() {
    for (minute, expected) in [
        (0, Ok("T01:00")),
        (1, Ok("T01:01")),
        (59, Ok("T01:59")),
        (60, Err(EncodeError::InvalidMinute)),
        (u8::MAX, Err(EncodeError::InvalidMinute)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_hour(1).unwrap();
        match expected {
            Ok(s) => {
                appender.append_minute(minute).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()), "minute={minute}");
            }
            Err(e) => {
                assert_eq!(
                    appender.append_minute(minute).err(),
                    Some(e),
                    "minute={minute}"
                );
            }
        }
    }
}

#[test]
fn hour_minute_second() {
    for (second, expected) in [
        (0, Ok("T01:23:00")),
        (1, Ok("T01:23:01")),
        (59, Ok("T01:23:59")),
        (60, Ok("T01:23:60")),
        (61, Err(EncodeError::InvalidSecond)),
        (u8::MAX, Err(EncodeError::InvalidSecond)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_hour(1).unwrap().append_minute(23).unwrap();
        match expected {
            Ok(s) => {
                appender.append_second(second).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()), "second={second}");
            }
            Err(e) => {
                assert_eq!(
                    appender.append_second(second).err(),
                    Some(e),
                    "second={second}"
                );
            }
        }
    }
}

#[test]
fn hour_minute_millisecond() {
    for (millisecond, expected) in [
        (0, Ok("T01:23:00.000")),
        (1, Ok("T01:23:00.001")),
        (59_999, Ok("T01:23:59.999")),
        (60_999, Ok("T01:23:60.999")),
        (61_000, Err(EncodeError::InvalidMillisecond)),
        (u32::MAX, Err(EncodeError::InvalidMillisecond)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_hour(1).unwrap().append_minute(23).unwrap();
        match expected {
            Ok(s) => {
                appender.append_millisecond(millisecond).unwrap();
                assert_eq!(
                    encoder.into_string(),
                    Ok(s.to_string()),
                    "millisecond={millisecond}"
                );
            }
            Err(e) => {
                assert_eq!(
                    appender.append_millisecond(millisecond).err(),
                    Some(e),
                    "millisecond={millisecond}"
                );
            }
        }
    }
}

#[test]
fn hour_minute_microsecond() {
    for (microsecond, expected) in [
        (0, Ok("T01:23:00.000_000")),
        (1, Ok("T01:23:00.000_001")),
        (59_999_999, Ok("T01:23:59.999_999")),
        (60_999_999, Ok("T01:23:60.999_999")),
        (61_000_000, Err(EncodeError::InvalidMicrosecond)),
        (u32::MAX, Err(EncodeError::InvalidMicrosecond)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_hour(1).unwrap().append_minute(23).unwrap();
        match expected {
            Ok(s) => {
                appender.append_microsecond(microsecond).unwrap();
                assert_eq!(
                    encoder.into_string(),
                    Ok(s.to_string()),
                    "microsecond={microsecond}"
                );
            }
            Err(e) => {
                assert_eq!(
                    appender.append_microsecond(microsecond).err(),
                    Some(e),
                    "microsecond={microsecond}"
                );
            }
        }
    }
}

#[test]
fn hour_minute_nanosecond() {
    for (nanosecond, expected) in [
        (0, Ok("T01:23:00.000_000_000")),
        (1, Ok("T01:23:00.000_000_001")),
        (59_999_999_999, Ok("T01:23:59.999_999_999")),
        (60_999_999_999, Ok("T01:23:60.999_999_999")),
        (61_000_000_000, Err(EncodeError::InvalidNanosecond)),
        (u64::MAX, Err(EncodeError::InvalidNanosecond)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_hour(1).unwrap().append_minute(23).unwrap();
        match expected {
            Ok(s) => {
                appender.append_nanosecond(nanosecond).unwrap();
                assert_eq!(
                    encoder.into_string(),
                    Ok(s.to_string()),
                    "nanosecond={nanosecond}"
                );
            }
            Err(e) => {
                assert_eq!(
                    appender.append_nanosecond(nanosecond).err(),
                    Some(e),
                    "nanosecond={nanosecond}"
                );
            }
        }
    }
}

#[test]
fn date_tzoffset() {
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001Z"
    );
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-02Z"
    );
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_day(3)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-02-03Z"
    );
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_week(2)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-W02Z"
    );
}
#[test]
fn datetime_tzoffset() {
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_day(3)
            .unwrap()
            .append_hour(4)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-02-03T04Z"
    );
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_day(3)
            .unwrap()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-02-03T04:05Z"
    );
    assert_eq!(
        Encoder::new()
            .append_year(2001)
            .unwrap()
            .append_month(2)
            .unwrap()
            .append_day(3)
            .unwrap()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_second(6)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "D2001-02-03T04:05:06Z"
    );
}
#[test]
fn time_tzoffset() {
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04Z"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05Z"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_second(6)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05:06Z"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_millisecond(6007)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05:06.007Z"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_microsecond(6_007_008)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05:06.007_008Z"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_nanosecond(6_007_008_009)
            .unwrap()
            .append_tzoffset(0, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05:06.007_008_009Z"
    );
}
#[test]
fn tzoffset() {
    for (hour, minute, expected) in [
        (-13, 0, Err(EncodeError::InvalidTimezoneOffset)),
        (0, 60, Err(EncodeError::InvalidTimezoneOffset)),
        (-12, 59, Ok("D2021-1259")),
        (-12, 0, Ok("D2021-12")),
        (-5, 30, Ok("D2021-0530")),
        (-1, 0, Ok("D2021-01")),
        (-1, 1, Ok("D2021-0101")),
        // Cannot represent -00xx.
        (0, 0, Ok("D2021Z")),
        (0, 1, Ok("D2021+0001")),
        (1, 0, Ok("D2021+01")),
        (5, 30, Ok("D2021+0530")),
        (12, 59, Ok("D2021+1259")),
        (13, 0, Err(EncodeError::InvalidTimezoneOffset)),
        (0, 60, Err(EncodeError::InvalidTimezoneOffset)),
        (i8::MAX, 0, Err(EncodeError::InvalidTimezoneOffset)),
        (0, u8::MAX, Err(EncodeError::InvalidTimezoneOffset)),
    ] {
        let mut encoder = Encoder::new();
        let appender = encoder.append_year(2021).unwrap();
        match expected {
            Ok(s) => {
                appender.append_tzoffset(hour, minute).unwrap();
                assert_eq!(encoder.into_string(), Ok(s.to_string()), "minute={minute}");
            }
            Err(e) => {
                assert_eq!(
                    appender.append_tzoffset(hour, minute).err(),
                    Some(e),
                    "minute={minute}"
                );
            }
        }
    }
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_tzoffset(-8, 0)
            .unwrap()
            .as_str()
            .unwrap(),
        "T04-08"
    );
    assert_eq!(
        Encoder::new()
            .append_hour(4)
            .unwrap()
            .append_minute(5)
            .unwrap()
            .append_nanosecond(6_007_008_009)
            .unwrap()
            .append_tzoffset(5, 30) // India Standard Time
            .unwrap()
            .as_str()
            .unwrap(),
        "T04:05:06.007_008_009+0530"
    );
}
