use jtoo::{EncodeError, Encoder};

#[test]
fn empty() {
    let encoder = Encoder::new();
    assert_eq!(encoder.to_string(), Ok("".to_string()));
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
    assert_eq!(encoder.to_string(), Ok("[T,F]".to_string()));
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
    assert_eq!(encoder.to_string(), Ok("B".to_string()));
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
        encoder.to_string(),
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
    assert_eq!(encoder.to_string(), Ok("[B,T]".to_string()));
}

#[test]
fn byte_string_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_byte_string().unwrap();
    assert_eq!(
        encoder.append_bool(true),
        Err(EncodeError::UnclosedByteString)
    );
    assert_eq!(encoder.to_string(), Err(EncodeError::UnclosedByteString));
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
        (1234, 0, "1_234.0"),
        (-1234, 0, "-1_234.0"),
        (12345, 0, "12_345.0"),
        (-12345, 0, "-12_345.0"),
        (123456, 0, "123_456.0"),
        (-123456, 0, "-123_456.0"),
        (1234567, 0, "1_234_567.0"),
        (-1234567, 0, "-1_234_567.0"),
        (i64::MAX, 0, "9_223_372_036_854_775_807.0"),
        (i64::MIN, 0, "-9_223_372_036_854_775_808.0"),
        // Zeros added after the decimal point.
        (1, -2, "0.01"),
        (-1, -2, "-0.01"),
        (1, -3, "0.001"),
        (1, -4, "0.000_1"),
        (1, -5, "0.000_01"),
        (1, -6, "0.000_001"),
        (1, -7, "0.000_000_1"),
        (-1, -7, "-0.000_000_1"),
        (1234, -7, "0.000_123_4"),
        (-1234, -7, "-0.000_123_4"),
        // No zeros added after decimal.
        (1, -1, "0.1"),
        (-1, -1, "-0.1"),
        (12, -2, "0.12"),
        (123, -3, "0.123"),
        (1234, -4, "0.123_4"),
        (12345, -5, "0.123_45"),
        (123456, -6, "0.123_456"),
        (1234567, -7, "0.123_456_7"),
        (-1234567, -7, "-0.123_456_7"),
        (i64::MAX, -19, "0.922_337_203_685_477_580_7"),
        (i64::MIN, -19, "-0.922_337_203_685_477_580_8"),
        // Digits on both sides of decimal point.
        (12, -1, "1.2"),
        (123, -1, "12.3"),
        (1234, -1, "123.4"),
        (12345, -1, "1_234.5"),
        (123456, -2, "1_234.56"),
        (1234567, -3, "1_234.567"),
        (12345678, -4, "1_234.567_8"),
        (12345678, -5, "123.456_78"),
        (12345678, -6, "12.345_678"),
        (12345678, -7, "1.234_567_8"),
        (i64::MAX, -18, "9.223_372_036_854_775_807"),
        (i64::MIN, -18, "-9.223_372_036_854_775_808"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_decimal(value, exp).unwrap();
        assert_eq!(
            encoder.to_string(),
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
        (1000, "1_000"),
        (10000, "10_000"),
        (100000, "100_000"),
        (1000000, "1_000_000"),
        (-1000000, "-1_000_000"),
        (-100000, "-100_000"),
        (-10000, "-10_000"),
        (-1000, "-1_000"),
        (-100, "-100"),
        (-10, "-10"),
        (-1, "-1"),
        (i64::MAX, "9_223_372_036_854_775_807"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_integer(value).unwrap();
        assert_eq!(encoder.to_string(), Ok(expected.to_string()));
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
    assert_eq!(encoder.to_string(), Ok("[]".to_string()));
}

#[test]
fn list_nested() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.to_string(), Ok("[[]]".to_string()));
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
        encoder.to_string(),
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
    assert_eq!(encoder.to_string(), Ok("[[],T]".to_string()));
}

#[test]
fn list_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.to_string(), Err(EncodeError::UnclosedList));
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
    assert_eq!(encoder.to_string(), Ok("\"\"".to_string()));
}

#[test]
fn string_value() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_string("string1").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.to_string(), Ok("\"string1\"".to_string()));
}

#[test]
fn string_closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    encoder.close_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.to_string(), Ok("[\"\",T]".to_string()));
}

#[test]
fn string_unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(true), Err(EncodeError::UnclosedString));
    assert_eq!(encoder.to_string(), Err(EncodeError::UnclosedString));
}
