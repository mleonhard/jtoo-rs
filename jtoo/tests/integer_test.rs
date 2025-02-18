use jtoo::{Encode, EncodeError, Encoder};

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
