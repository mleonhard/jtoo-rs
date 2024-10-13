use jtoo::{Encode, EncodeError, Encoder};
use jtoo_derive::Encode;

#[test]
fn simple_struct() {
    #[derive(Encode)]
    pub struct Struct0 {
        pub field0: bool,
    }
    let value = Struct0 { field0: true };
    assert_eq!(value.encode(), Ok("[[\"field0\",T]]".to_string()));
}

#[test]
fn encode_basic_types() {
    #[derive(Encode)]
    pub struct Struct0 {
        pub field0: bool,
        pub field1: i8,
        pub field2: u8,
        pub field3: i16,
        pub field4: u16,
        pub field5: i32,
        pub field6: u32,
        pub field7: i64,
        pub field8: u64,
        pub field9: &'static str,
        pub field10: Box<str>,
        pub field11: String,
        pub field12: &'static [u8],
        pub field13: Box<[u8]>,
        pub field14: Vec<u8>,
        pub field15: Option<bool>,
        pub field16: &'static [bool],
        pub field17: Box<[bool]>,
        pub field18: Vec<bool>,
    }
    let value = Struct0 {
        field0: true,
        field1: -1,
        field2: 1,
        field3: -2,
        field4: 2,
        field5: -3,
        field6: 3,
        field7: -4,
        field8: 4,
        field9: "5",
        field10: "5".to_string().into_boxed_str(),
        field11: "6".to_string(),
        field12: &[7, 8],
        field13: vec![8, 9].into_boxed_slice(),
        field14: vec![9, 10],
        field15: Some(true),
        field16: &[true, false],
        field17: vec![false, true].into_boxed_slice(),
        field18: vec![true, false, true],
    };
    assert_eq!(value.encode(), Ok("".to_string()));
}

#[test]
fn encode_ok() {
    struct Struct;
    // TODO: Derive Encode.
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
