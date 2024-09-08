use jtoo::{Encode, EncodeError, Encoder};

#[test]
fn encode_ok() {
    struct Struct;
    impl Encode for Struct {
        fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
            encoder.open_string()?;
            encoder.append_string("string1")?;
            encoder.close_string()
        }
    }
    assert_eq!(Struct {}.encode(), Ok("\"string1\"".to_string()));
}

#[test]
fn encode_err() {
    struct Struct;
    impl Encode for Struct {
        fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
            encoder.open_string()
        }
    }
    assert_eq!(Struct {}.encode(), Err(EncodeError::UnclosedString));
}
