#![cfg(feature = "time")]
use jtoo::{Decode, Encode, EncodeError, ErrorReason};
use time::{Date, Month, OffsetDateTime, Time, UtcOffset};

#[test]
fn decode() {
    assert_eq!(
        OffsetDateTime::decode(b"D0001-01-01T00:00:00-2459")
            .unwrap_err()
            .reason,
        ErrorReason::TimezoneOffsetHourOutOfRange
    );
    assert_eq!(
        OffsetDateTime::decode(b"D0000-01-01T00:00:00-2359")
            .unwrap_err()
            .reason,
        ErrorReason::YearOutOfRange
    );
    assert_eq!(
        OffsetDateTime::decode(b"D0001-01-01T00:00:00-2359").unwrap(),
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(1, Month::January, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
            UtcOffset::from_hms(-23, 59, 0).unwrap()
        )
    );
    assert_eq!(
        OffsetDateTime::decode(b"D1970-01-01T00:00:00Z").unwrap(),
        OffsetDateTime::UNIX_EPOCH
    );
    assert_eq!(
        OffsetDateTime::decode(b"D2025-02-17T02:03:04+0506").unwrap(),
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2025, Month::February, 17).unwrap(),
            Time::from_hms(2, 3, 4).unwrap(),
            UtcOffset::from_hms(5, 6, 0).unwrap()
        )
    );
    assert_eq!(
        OffsetDateTime::decode(b"D9999-12-31T23:59:59.999_999_999+2359").unwrap(),
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(9999, Month::December, 31).unwrap(),
            Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            UtcOffset::from_hms(23, 59, 0).unwrap()
        )
    );
    assert_eq!(
        OffsetDateTime::decode(b"D9999-12-31T23:59:59.999_999_999+2459")
            .unwrap_err()
            .reason,
        ErrorReason::TimezoneOffsetHourOutOfRange
    );
}

#[test]
fn encode() {
    use time::{Date, Month, OffsetDateTime, Time, UtcOffset};
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(1, Month::January, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
            UtcOffset::from_hms(-24, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(0, Month::January, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
            UtcOffset::from_hms(-23, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidYear)
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(1, Month::January, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
            UtcOffset::from_hms(-23, 59, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D0001-01-01T00:00:00-2359"
    );
    assert_eq!(
        OffsetDateTime::UNIX_EPOCH.encode().unwrap().as_str(),
        "D1970-01-01T00:00:00Z"
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(2025, Month::February, 17).unwrap(),
            Time::from_hms(2, 3, 4).unwrap(),
            UtcOffset::from_hms(5, 6, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D2025-02-17T02:03:04+0506"
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(9999, Month::December, 31).unwrap(),
            Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            UtcOffset::from_hms(23, 59, 0).unwrap()
        )
        .encode()
        .unwrap()
        .as_str(),
        "D9999-12-31T23:59:59.999_999_999+2359"
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(9999, Month::December, 31).unwrap(),
            Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            UtcOffset::from_hms(24, 59, 0).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
    assert_eq!(
        OffsetDateTime::new_in_offset(
            Date::from_calendar_date(9999, Month::December, 31).unwrap(),
            Time::from_hms_nano(23, 59, 59, 999_999_999).unwrap(),
            UtcOffset::from_hms(23, 59, 1).unwrap()
        )
        .encode(),
        Err(EncodeError::InvalidOffset)
    );
}
