use jtoo::{DateTimeOffset, Encode, EncodeError, Encoder};

#[test]
fn millisecond() {
    assert_eq!(999, DateTimeOffset::MAX.millisecond());
}

#[test]
fn microsecond() {
    assert_eq!(999_999, DateTimeOffset::MAX.microsecond());
}

#[test]
fn encode() {
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(1, time::Month::January, 1).unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
            time::UtcOffset::from_hms(-24, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(0, time::Month::January, 1).unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
            time::UtcOffset::from_hms(-23, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidYear)
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(1, time::Month::January, 1).unwrap(),
            time::Time::from_hms(0, 0, 0).unwrap(),
            time::UtcOffset::from_hms(-23, 59, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D0001-01-01T00:00:00-2359"
    );
    assert_eq!(
        time::OffsetDateTime::UNIX_EPOCH.encode().unwrap().as_str(),
        "D1970-01-01T00:00:00Z"
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(2025, time::Month::February, 17).unwrap(),
            time::Time::from_hms(2, 3, 4).unwrap(),
            time::UtcOffset::from_hms(5, 6, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D2025-02-17T02:03:04+0506"
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(9999, time::Month::December, 31).unwrap(),
            time::Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            time::UtcOffset::from_hms(23, 59, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D9999-12-31T23:59:59.999_999_999+2359"
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(9999, time::Month::December, 31).unwrap(),
            time::Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            time::UtcOffset::from_hms(24, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
    assert_eq!(
        time::OffsetDateTime::new_in_offset(
            time::Date::from_calendar_date(9999, time::Month::December, 31).unwrap(),
            time::Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            time::UtcOffset::from_hms(23, 59, 1).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
}

#[test]
fn append() {
    for (
        year,
        month,
        day,
        hour,
        minute,
        second,
        nanosecond,
        offset_hour,
        offset_minute,
        expected,
    ) in [
        (0, 1, 1, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidYear)),
        (10000, 1, 1, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidYear)),
        (1, 0, 1, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidMonth)),
        (1, 13, 1, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidMonth)),
        (1, 1, 0, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidDay)),
        (1, 1, 32, 0, 0, 0, 0, 0, 0, Err(EncodeError::InvalidDay)),
        (1, 1, 1, 24, 0, 0, 0, 0, 0, Err(EncodeError::InvalidHour)),
        (1, 1, 1, 0, 60, 0, 0, 0, 0, Err(EncodeError::InvalidMinute)),
        (1, 1, 1, 0, 0, 61, 0, 0, 0, Err(EncodeError::InvalidSecond)),
        (
            1,
            1,
            1,
            0,
            0,
            0,
            1_000_000_000,
            0,
            0,
            Err(EncodeError::InvalidNanosecond),
        ),
        (1, 1, 1, 0, 0, 0, 0, -24, 0, Err(EncodeError::InvalidOffset)),
        (1, 1, 1, 0, 0, 0, 0, 24, 0, Err(EncodeError::InvalidOffset)),
        (1, 1, 1, 0, 0, 0, 0, 0, 60, Err(EncodeError::InvalidOffset)),
        (1, 1, 1, 0, 0, 0, 0, 0, 0, Ok("D0001-01-01T00:00:00Z")),
        (
            1,
            1,
            1,
            0,
            0,
            0,
            123_000_000,
            0,
            0,
            Ok("D0001-01-01T00:00:00.123Z"),
        ),
        (
            1,
            1,
            1,
            0,
            0,
            0,
            123_456_000,
            0,
            0,
            Ok("D0001-01-01T00:00:00.123_456Z"),
        ),
        (
            1,
            1,
            1,
            0,
            0,
            0,
            123_456_789,
            0,
            0,
            Ok("D0001-01-01T00:00:00.123_456_789Z"),
        ),
        (
            9999,
            12,
            31,
            23,
            59,
            60,
            999_999_999,
            0,
            0,
            Ok("D9999-12-31T23:59:60.999_999_999Z"),
        ),
        (
            1,
            1,
            1,
            0,
            0,
            0,
            0,
            -23,
            59,
            Ok("D0001-01-01T00:00:00-2359"),
        ),
        (1, 1, 1, 0, 0, 0, 0, 23, 59, Ok("D0001-01-01T00:00:00+2359")),
    ] {
        let mut encoder = Encoder::new();
        match expected {
            Ok(s) => {
                encoder
                    .append_date_time_offset(
                        year,
                        month,
                        day,
                        hour,
                        minute,
                        second,
                        nanosecond,
                        offset_hour,
                        offset_minute,
                    )
                    .expect(s);
                assert_eq!(encoder.as_str(), Ok(s));
            }
            Err(e) => {
                assert_eq!(
                    encoder
                        .append_date_time_offset(
                            year,
                            month,
                            day,
                            hour,
                            minute,
                            second,
                            nanosecond,
                            offset_hour,
                            offset_minute,
                        )
                        .err(),
                    Some(e)
                );
            }
        }
    }
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(
        encoder.append_date_time_offset(1, 1, 1, 0, 0, 0, 0, 0, 0),
        Err(EncodeError::UnclosedString)
    );
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder
        .append_date_time_offset(1, 1, 1, 0, 0, 0, 0, 0, 0)
        .unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[D0001-01-01T00:00:00Z,T]"));
}
