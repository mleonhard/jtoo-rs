use jtoo::{Decimal, Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

#[cfg(feature = "rust_decimal")]
#[test]
fn encode_rust_decimal() {
    use rust_decimal_macros::dec;
    assert_eq!(
        dec!(-9_223_372_036_854_775_809).encode(),
        Err(EncodeError::OutOfRange)
    );
    assert_eq!(
        dec!(-9_223_372_036_854_775_808).encode().unwrap().as_str(),
        "-9_223_372_036_854_775_808.0"
    );
    assert_eq!(dec!(-1).encode().unwrap().as_str(), "-1.0");
    assert_eq!(
        dec!(-0.000_000_000_000_000_000_000_000_000_1)
            .encode()
            .unwrap()
            .as_str(),
        "-0.000_000_000_000_000_000_000_000_000_1"
    );
    assert_eq!(dec!(0).encode().unwrap().as_str(), "0.0");
    assert_eq!(
        dec!(0.000_000_000_000_000_000_000_000_000_1)
            .encode()
            .unwrap()
            .as_str(),
        "0.000_000_000_000_000_000_000_000_000_1"
    );
    assert_eq!(dec!(1).encode().unwrap().as_str(), "1.0");
    assert_eq!(
        dec!(9_223_372_036_854_775_807).encode().unwrap().as_str(),
        "9_223_372_036_854_775_807.0"
    );
    assert_eq!(
        dec!(9_223_372_036_854_775_808).encode(),
        Err(EncodeError::OutOfRange)
    );
}

#[cfg(feature = "rust_decimal")]
#[test]
fn decode_rust_decimal() {
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    assert_eq!(
        Decimal::decode(b"-9_223_372_036_854_775_809.0")
            .unwrap_err()
            .reason,
        ErrorReason::DecimalMantissaOutOfRange
    );
    assert_eq!(
        Decimal::decode(b"-9_223_372_036_854_775_808.0").unwrap(),
        dec!(-9_223_372_036_854_775_808)
    );
    assert_eq!(Decimal::decode(b"-1.0").unwrap(), dec!(-1));
    assert_eq!(
        Decimal::decode(b"-0.000_000_000_000_000_000_000_000_000_1").unwrap(),
        dec!(-0.000_000_000_000_000_000_000_000_000_1)
    );
    assert_eq!(Decimal::decode(b"0.0").unwrap(), dec!(0));
    assert_eq!(
        Decimal::decode(b"0.000_000_000_000_000_000_000_000_000_1").unwrap(),
        dec!(0.000_000_000_000_000_000_000_000_000_1)
    );
    assert_eq!(Decimal::decode(b"1.0").unwrap(), dec!(1));
    assert_eq!(
        Decimal::decode(b"9_223_372_036_854_775_807.0").unwrap(),
        dec!(9_223_372_036_854_775_807)
    );
    assert_eq!(
        Decimal::decode(b"9_223_372_036_854_775_808.0")
            .unwrap_err()
            .reason,
        ErrorReason::DecimalMantissaOutOfRange
    );
}

#[test]
fn append() {
    for (decimal, string) in [
        // Exponent is zero.
        (Decimal::new(1, 0), "1.0"),
        (Decimal::new(-1, 0), "-1.0"),
        (Decimal::new(12, 0), "12.0"),
        (Decimal::new(-12, 0), "-12.0"),
        (Decimal::new(123, 0), "123.0"),
        (Decimal::new(-123, 0), "-123.0"),
        (Decimal::new(1_234, 0), "1_234.0"),
        (Decimal::new(-1_234, 0), "-1_234.0"),
        (Decimal::new(12_345, 0), "12_345.0"),
        (Decimal::new(-12_345, 0), "-12_345.0"),
        (Decimal::new(123_456, 0), "123_456.0"),
        (Decimal::new(-123_456, 0), "-123_456.0"),
        (Decimal::new(1_234_567, 0), "1_234_567.0"),
        (Decimal::new(-1_234_567, 0), "-1_234_567.0"),
        (Decimal::new(i64::MAX, 0), "9_223_372_036_854_775_807.0"),
        (Decimal::new(i64::MIN, 0), "-9_223_372_036_854_775_808.0"),
        // Zeros before decimal point.
        (Decimal::new(0, 0), "0.0"),
        (Decimal::new(1, 0), "1.0"),
        (Decimal::new(10, 0), "10.0"),
        (Decimal::new(100, 0), "100.0"),
        (Decimal::new(1_000, 0), "1_000.0"),
        (Decimal::new(10_000, 0), "10_000.0"),
        (Decimal::new(100_000, 0), "100_000.0"),
        (Decimal::new(1_000_000, 0), "1_000_000.0"),
        (Decimal::new(10_000_000, 0), "10_000_000.0"),
        (Decimal::new(-10_000_000, 0), "-10_000_000.0"),
        (Decimal::new(120_000_000, 0), "120_000_000.0"),
        (Decimal::new(1_230_000_000, 0), "1_230_000_000.0"),
        (Decimal::new(12_340_000_000, 0), "12_340_000_000.0"),
        (Decimal::new(-12_340_000_000, 0), "-12_340_000_000.0"),
        // Zeros after the decimal point.
        (Decimal::new(0, 0), "0.0"),
        (Decimal::new(0, 1), "0.0"),
        (Decimal::new(0, 2), "0.00"),
        (Decimal::new(0, 3), "0.000"),
        (Decimal::new(0, 4), "0.000_0"),
        (Decimal::new(0, 5), "0.000_00"),
        (Decimal::new(0, 6), "0.000_000"),
        (Decimal::new(0, 7), "0.000_000_0"),
        (Decimal::new(1, 2), "0.01"),
        (Decimal::new(-1, 2), "-0.01"),
        (Decimal::new(10, 2), "0.10"),
        (Decimal::new(1, 3), "0.001"),
        (Decimal::new(1, 4), "0.000_1"),
        (Decimal::new(10, 4), "0.001_0"),
        (Decimal::new(100, 4), "0.010_0"),
        (Decimal::new(1_000, 4), "0.100_0"),
        (Decimal::new(10_000, 4), "1.000_0"),
        (Decimal::new(1, 5), "0.000_01"),
        (Decimal::new(1, 6), "0.000_001"),
        (Decimal::new(1, 7), "0.000_000_1"),
        (Decimal::new(-1, 7), "-0.000_000_1"),
        (Decimal::new(1_234, 7), "0.000_123_4"),
        (Decimal::new(-1_234, 7), "-0.000_123_4"),
        // Digits after decimal.
        (Decimal::new(1, 1), "0.1"),
        (Decimal::new(-1, 1), "-0.1"),
        (Decimal::new(12, 2), "0.12"),
        (Decimal::new(123, 3), "0.123"),
        (Decimal::new(1_234, 4), "0.123_4"),
        (Decimal::new(12_345, 5), "0.123_45"),
        (Decimal::new(123_456, 6), "0.123_456"),
        (Decimal::new(1_234_567, 7), "0.123_456_7"),
        (Decimal::new(-1_234_567, 7), "-0.123_456_7"),
        (Decimal::new(i64::MAX, 19), "0.922_337_203_685_477_580_7"),
        (Decimal::new(i64::MIN, 19), "-0.922_337_203_685_477_580_8"),
        // Digits on both sides of decimal point.
        (Decimal::new(10, 1), "1.0"),
        (Decimal::new(12, 1), "1.2"),
        (Decimal::new(123, 1), "12.3"),
        (Decimal::new(1_234, 1), "123.4"),
        (Decimal::new(12_345, 1), "1_234.5"),
        (Decimal::new(10, 2), "0.10"),
        (Decimal::new(123_456, 2), "1_234.56"),
        (Decimal::new(1_234_567, 3), "1_234.567"),
        (Decimal::new(12_345_678, 4), "1_234.567_8"),
        (Decimal::new(12_345_678, 5), "123.456_78"),
        (Decimal::new(12_345_678, 6), "12.345_678"),
        (Decimal::new(12_345_678, 7), "1.234_567_8"),
        (Decimal::new(i64::MAX, 18), "9.223_372_036_854_775_807"),
        (Decimal::new(i64::MIN, 18), "-9.223_372_036_854_775_808"),
    ] {
        let mut encoder = Encoder::new();
        encoder.append_decimal(decimal).unwrap();
        assert_eq!(
            encoder.as_str(),
            Ok(string),
            "decimal={decimal} string={string}"
        );
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn consume() {
    // Zeroes before decimal point.
    assert_eq!(
        Decoder::new(b"0.0").consume_decimal(),
        Ok(Decimal::new(0, 0))
    );
    assert_eq!(
        Decoder::new(b"1.0").consume_decimal(),
        Ok(Decimal::new(1, 0))
    );
    assert_eq!(
        Decoder::new(b"10.0").consume_decimal(),
        Ok(Decimal::new(10, 0))
    );
    assert_eq!(
        Decoder::new(b"100.0").consume_decimal(),
        Ok(Decimal::new(100, 0))
    );
    assert_eq!(
        Decoder::new(b"1_000.0").consume_decimal(),
        Ok(Decimal::new(1_000, 0))
    );
    assert_eq!(
        Decoder::new(b"10_000.0").consume_decimal(),
        Ok(Decimal::new(10_000, 0))
    );
    assert_eq!(
        Decoder::new(b"100_000.0").consume_decimal(),
        Ok(Decimal::new(100_000, 0))
    );
    assert_eq!(
        Decoder::new(b"1_000_000.0").consume_decimal(),
        Ok(Decimal::new(1_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"10_000_000.0").consume_decimal(),
        Ok(Decimal::new(10_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"-10_000_000.0").consume_decimal(),
        Ok(Decimal::new(-10_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"120_000_000.0").consume_decimal(),
        Ok(Decimal::new(120_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"1_230_000_000.0").consume_decimal(),
        Ok(Decimal::new(1_230_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"12_340_000_000.0").consume_decimal(),
        Ok(Decimal::new(12_340_000_000, 0))
    );
    assert_eq!(
        Decoder::new(b"-12_340_000_000.0").consume_decimal(),
        Ok(Decimal::new(-12_340_000_000, 0))
    );
    // Exponent is zero.
    assert_eq!(
        Decoder::new(b"1.0").consume_decimal(),
        Ok(Decimal::new(1, 0))
    );
    assert_eq!(
        Decoder::new(b"-1.0").consume_decimal(),
        Ok(Decimal::new(-1, 0))
    );
    assert_eq!(
        Decoder::new(b"12.0").consume_decimal(),
        Ok(Decimal::new(12, 0))
    );
    assert_eq!(
        Decoder::new(b"-12.0").consume_decimal(),
        Ok(Decimal::new(-12, 0))
    );
    assert_eq!(
        Decoder::new(b"123.0").consume_decimal(),
        Ok(Decimal::new(123, 0))
    );
    assert_eq!(
        Decoder::new(b"-123.0").consume_decimal(),
        Ok(Decimal::new(-123, 0))
    );
    assert_eq!(
        Decoder::new(b"1_234.0").consume_decimal(),
        Ok(Decimal::new(1_234, 0))
    );
    assert_eq!(
        Decoder::new(b"-1_234.0").consume_decimal(),
        Ok(Decimal::new(-1_234, 0))
    );
    assert_eq!(
        Decoder::new(b"12_345.0").consume_decimal(),
        Ok(Decimal::new(12_345, 0))
    );
    assert_eq!(
        Decoder::new(b"-12_345.0").consume_decimal(),
        Ok(Decimal::new(-12_345, 0))
    );
    assert_eq!(
        Decoder::new(b"123_456.0").consume_decimal(),
        Ok(Decimal::new(123_456, 0))
    );
    assert_eq!(
        Decoder::new(b"-123_456.0").consume_decimal(),
        Ok(Decimal::new(-123_456, 0))
    );
    assert_eq!(
        Decoder::new(b"1_234_567.0").consume_decimal(),
        Ok(Decimal::new(1_234_567, 0))
    );
    assert_eq!(
        Decoder::new(b"-1_234_567.0").consume_decimal(),
        Ok(Decimal::new(-1_234_567, 0))
    );
    assert_eq!(
        Decoder::new(b"9_223_372_036_854_775_807.0").consume_decimal(),
        Ok(Decimal::new(i64::MAX, 0))
    );
    assert_eq!(
        Decoder::new(b"-9_223_372_036_854_775_808.0").consume_decimal(),
        Ok(Decimal::new(i64::MIN, 0))
    );
    // Zeros after the decimal point.
    assert_eq!(
        Decoder::new(b"0.0").consume_decimal(),
        Ok(Decimal::new(0, 0))
    );
    assert_eq!(
        Decoder::new(b"0.00").consume_decimal(),
        Ok(Decimal::new(0, 2))
    );
    assert_eq!(
        Decoder::new(b"0.01").consume_decimal(),
        Ok(Decimal::new(1, 2))
    );
    assert_eq!(
        Decoder::new(b"-0.01").consume_decimal(),
        Ok(Decimal::new(-1, 2))
    );
    assert_eq!(
        Decoder::new(b"0.10").consume_decimal(),
        Ok(Decimal::new(10, 2))
    );
    assert_eq!(
        Decoder::new(b"0.001").consume_decimal(),
        Ok(Decimal::new(1, 3))
    );
    assert_eq!(
        Decoder::new(b"0.000_1").consume_decimal(),
        Ok(Decimal::new(1, 4))
    );
    assert_eq!(
        Decoder::new(b"0.001_0").consume_decimal(),
        Ok(Decimal::new(10, 4))
    );
    assert_eq!(
        Decoder::new(b"0.010_0").consume_decimal(),
        Ok(Decimal::new(100, 4))
    );
    assert_eq!(
        Decoder::new(b"0.100_0").consume_decimal(),
        Ok(Decimal::new(1_000, 4))
    );
    assert_eq!(
        Decoder::new(b"1.000_0").consume_decimal(),
        Ok(Decimal::new(10_000, 4))
    );
    assert_eq!(
        Decoder::new(b"0.000_01").consume_decimal(),
        Ok(Decimal::new(1, 5))
    );
    assert_eq!(
        Decoder::new(b"0.000_001").consume_decimal(),
        Ok(Decimal::new(1, 6))
    );
    assert_eq!(
        Decoder::new(b"0.000_000_1").consume_decimal(),
        Ok(Decimal::new(1, 7))
    );
    assert_eq!(
        Decoder::new(b"-0.000_000_1").consume_decimal(),
        Ok(Decimal::new(-1, 7))
    );
    assert_eq!(
        Decoder::new(b"0.000_123_4").consume_decimal(),
        Ok(Decimal::new(1_234, 7))
    );
    assert_eq!(
        Decoder::new(b"-0.000_123_4").consume_decimal(),
        Ok(Decimal::new(-1_234, 7))
    );
    // No zeros added after decimal.
    assert_eq!(
        Decoder::new(b"0.1").consume_decimal(),
        Ok(Decimal::new(1, 1))
    );
    assert_eq!(
        Decoder::new(b"-0.1").consume_decimal(),
        Ok(Decimal::new(-1, 1))
    );
    assert_eq!(
        Decoder::new(b"0.12").consume_decimal(),
        Ok(Decimal::new(12, 2))
    );
    assert_eq!(
        Decoder::new(b"0.123").consume_decimal(),
        Ok(Decimal::new(123, 3))
    );
    assert_eq!(
        Decoder::new(b"0.123_4").consume_decimal(),
        Ok(Decimal::new(1_234, 4))
    );
    assert_eq!(
        Decoder::new(b"0.123_45").consume_decimal(),
        Ok(Decimal::new(12_345, 5))
    );
    assert_eq!(
        Decoder::new(b"0.123_456").consume_decimal(),
        Ok(Decimal::new(123_456, 6))
    );
    assert_eq!(
        Decoder::new(b"0.123_456_7").consume_decimal(),
        Ok(Decimal::new(1_234_567, 7))
    );
    assert_eq!(
        Decoder::new(b"-0.123_456_7").consume_decimal(),
        Ok(Decimal::new(-1_234_567, 7))
    );
    assert_eq!(
        Decoder::new(b"0.922_337_203_685_477_580_7").consume_decimal(),
        Ok(Decimal::new(i64::MAX, 19))
    );
    assert_eq!(
        Decoder::new(b"-0.922_337_203_685_477_580_8").consume_decimal(),
        Ok(Decimal::new(i64::MIN, 19))
    );
    // Digits on both sides of decimal point.
    assert_eq!(
        Decoder::new(b"1.0").consume_decimal(),
        Ok(Decimal::new(1, 0))
    );
    assert_eq!(
        Decoder::new(b"1.2").consume_decimal(),
        Ok(Decimal::new(12, 1))
    );
    assert_eq!(
        Decoder::new(b"12.3").consume_decimal(),
        Ok(Decimal::new(123, 1))
    );
    assert_eq!(
        Decoder::new(b"123.4").consume_decimal(),
        Ok(Decimal::new(1_234, 1))
    );
    assert_eq!(
        Decoder::new(b"1_234.5").consume_decimal(),
        Ok(Decimal::new(12_345, 1))
    );
    assert_eq!(
        Decoder::new(b"0.10").consume_decimal(),
        Ok(Decimal::new(10, 2))
    );
    assert_eq!(
        Decoder::new(b"1_234.56").consume_decimal(),
        Ok(Decimal::new(123_456, 2))
    );
    assert_eq!(
        Decoder::new(b"1_234.567").consume_decimal(),
        Ok(Decimal::new(1_234_567, 3))
    );
    assert_eq!(
        Decoder::new(b"1_234.567_8").consume_decimal(),
        Ok(Decimal::new(12_345_678, 4))
    );
    assert_eq!(
        Decoder::new(b"123.456_78").consume_decimal(),
        Ok(Decimal::new(12_345_678, 5))
    );
    assert_eq!(
        Decoder::new(b"12.345_678").consume_decimal(),
        Ok(Decimal::new(12_345_678, 6))
    );
    assert_eq!(
        Decoder::new(b"1.234_567_8").consume_decimal(),
        Ok(Decimal::new(12_345_678, 7))
    );
    assert_eq!(
        Decoder::new(b"9.223_372_036_854_775_807").consume_decimal(),
        Ok(Decimal::new(i64::MAX, 18))
    );
    assert_eq!(
        Decoder::new(b"-9.223_372_036_854_775_808").consume_decimal(),
        Ok(Decimal::new(i64::MIN, 18))
    );
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(
        encoder.append_decimal(Decimal::ZERO),
        Err(EncodeError::UnclosedString)
    );
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_decimal(Decimal::new(300, 0)).unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[300.0,T]"));
}
