#![cfg(feature = "rust_decimal")]
use jtoo::{Decode, Encode, EncodeError, ErrorReason};

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
