use jtoo::{Encode, EncodeError, Encoder};

#[test]
fn encode_ok() {
    struct Struct;
    impl Encode for Struct {
        fn encode_using(&self, packer: &mut Encoder) -> Result<(), EncodeError> {
            packer.open_string()?;
            packer.append_string("string1")?;
            packer.close_string()
        }
    }
    assert_eq!(Struct {}.encode(), Ok("\"string1\"".to_string()));
}

#[test]
fn encode_err() {
    struct Struct;
    impl Encode for Struct {
        fn encode_using(&self, packer: &mut Encoder) -> Result<(), EncodeError> {
            packer.open_string()
        }
    }
    assert_eq!(Struct {}.encode(), Err(EncodeError::UnclosedString));
}
