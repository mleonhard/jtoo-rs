use jtoo::{Encode, EncodeError, Encoder};
use std::time::{Duration, SystemTime};

#[test]
fn encode() {
    for (duration, expected) in [
        (Duration::ZERO, "S0"),
        (Duration::from_secs(987_654_321), "S987_654_321"),
        (Duration::from_millis(987_654_321_100), "S987_654_321.100"),
        (Duration::from_millis(987_654_321_120), "S987_654_321.120"),
        (Duration::from_millis(987_654_321_123), "S987_654_321.123"),
        (
            Duration::from_micros(987_654_321_123_400),
            "S987_654_321.123_400",
        ),
        (
            Duration::from_micros(987_654_321_123_450),
            "S987_654_321.123_450",
        ),
        (
            Duration::from_micros(987_654_321_123_456),
            "S987_654_321.123_456",
        ),
        (
            Duration::from_nanos(987_654_321_123_456_700),
            "S987_654_321.123_456_700",
        ),
        (
            Duration::from_nanos(987_654_321_123_456_780),
            "S987_654_321.123_456_780",
        ),
        (
            Duration::from_nanos(987_654_321_123_456_789),
            "S987_654_321.123_456_789",
        ),
    ] {
        let msg = format!("{duration:?}");
        let timestamp = SystemTime::UNIX_EPOCH.checked_add(duration).expect(&msg);
        assert_eq!(timestamp.encode().expect(&msg).as_str(), expected);
    }
}

#[test]
fn seconds() {
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
                assert_eq!(encoder.as_str(), Ok(s));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_seconds(value), Err(e));
            }
        }
    }
}

#[test]
fn milliseconds() {
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
                assert_eq!(encoder.as_str(), Ok(s));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_milliseconds(value), Err(e));
            }
        }
    }
}

#[test]
fn microseconds() {
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
                assert_eq!(encoder.as_str(), Ok(s));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_microseconds(value), Err(e));
            }
        }
    }
}

#[test]
fn nanoseconds() {
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
                assert_eq!(encoder.as_str(), Ok(s));
            }
            Err(e) => {
                assert_eq!(encoder.append_timestamp_nanosecond(value), Err(e));
            }
        }
    }
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(
        encoder.append_timestamp_seconds(0),
        Err(EncodeError::UnclosedString)
    );
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_timestamp_seconds(0).unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[S0,T]"));
}
