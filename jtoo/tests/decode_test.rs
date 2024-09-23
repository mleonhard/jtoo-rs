use jtoo::{Decode, DecodeError, Decoder, ErrorReason};

#[test]
fn decode() {
    // TODO: Derive Decode.
    #[derive(Debug, Eq, PartialEq)]
    struct Struct(String);
    impl Decode for Struct {
        fn decode_using(decoder: &mut Decoder) -> Result<Self, DecodeError> {
            let value = decoder.consume_string()?;
            Ok(Self(value))
        }
    }
    assert_eq!(Struct::decode(b"\"str1\""), Ok(Struct("str1".to_string())));
    assert_eq!(
        format!("{:?}", Struct::decode(b"T").unwrap_err()),
        "DecodeError: ExpectedString: 'T'".to_string()
    );
    assert_eq!(
        Struct::decode(b"T").unwrap_err().reason,
        ErrorReason::ExpectedString
    );
}
