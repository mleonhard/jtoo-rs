use jtoo::{Decode, DecodeError, Decoder, ErrorReason};

#[test]
fn decode_using() {
    assert_eq!(bool::decode_using(&mut Decoder::new(b"T")), Ok(true));
}

#[test]
fn decode_error() {
    assert_eq!(
        format!(
            "{:?}",
            DecodeError {
                reason: ErrorReason::DataNotConsumed,
                debug_bytes: b"abc".to_vec()
            }
        ),
        "DecodeError: DataNotConsumed: 'abc'"
    );
}
