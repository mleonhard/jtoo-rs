use jtoo::{Encode, EncodeError, Encoder};
use jtoo_derive::Encode;

#[test]
fn simple_struct() {
    #[derive(Encode)]
    struct Struct0 {
        pub field0: bool,
    }
    let value = Struct0 { field0: true };
    assert_eq!(value.encode(), Ok("[[\"field0\",T]]".to_string()));
}

#[test]
fn basic_types() {
    #[derive(Encode)]
    struct Struct0 {
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
    assert_eq!(value.encode(), Ok(r#"[["field0",T],["field1",-1],["field2",1],["field3",-2],["field4",2],["field5",-3],["field6",3],["field7",-4],["field8",4],["field9","5"],["field10","5"],["field11","6"],["field12",[7,8]],["field13",[8,9]],["field14",[9,10]],["field15",[T]],["field16",[T,F]],["field17",[F,T]],["field18",[T,F,T]]]"#.to_string()));
}

#[test]
fn tuple_struct() {
    #[derive(Encode)]
    struct Struct0(bool, String, u8);
    let value = Struct0(true, String::from("value0"), 2);
    assert_eq!(value.encode(), Ok(r#"[T,"value0",2]"#.to_string()));
}

#[test]
fn unit_struct() {
    #[derive(Encode)]
    struct Struct0;
    let value = Struct0;
    assert_eq!(value.encode(), Ok(r#"[]"#.to_string()));
}

#[test]
fn nested_struct() {
    #[derive(Encode)]
    struct Struct0 {
        pub field0: bool,
    }
    #[derive(Encode)]
    struct Struct1(bool);
    #[derive(Encode)]
    struct Struct2;
    #[derive(Encode)]
    struct Struct3 {
        pub field0: Struct0,
        pub field1: Struct1,
        pub field2: Struct2,
    }
    let value = Struct3 {
        field0: Struct0 { field0: true },
        field1: Struct1(false),
        field2: Struct2 {},
    };
    assert_eq!(
        value.encode(),
        Ok(r#"[["field0",[["field0",T]]],["field1",[F]],["field2",[]]]"#.to_string())
    );
}

#[test]
fn encode_impl() {
    let mut encoder = Encoder::new();
    true.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("T"));

    let mut encoder = Encoder::new();
    (-42i8).encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("-42"));

    let mut encoder = Encoder::new();
    (-42i16).encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("-42"));

    let mut encoder = Encoder::new();
    (-42i32).encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("-42"));

    let mut encoder = Encoder::new();
    (-42i64).encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("-42"));

    let mut encoder = Encoder::new();
    42u8.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("42"));

    let mut encoder = Encoder::new();
    42u16.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("42"));

    let mut encoder = Encoder::new();
    42u32.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("42"));

    let mut encoder = Encoder::new();
    42u64.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("42"));

    let mut encoder = Encoder::new();
    "abc".encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("\"abc\""));

    let mut encoder = Encoder::new();
    "abc"
        .to_string()
        .into_boxed_str()
        .encode_using(&mut encoder)
        .unwrap();
    assert_eq!(encoder.as_str(), Ok("\"abc\""));

    let mut encoder = Encoder::new();
    "abc".to_string().encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("\"abc\""));

    let mut encoder = Encoder::new();
    Some(true).encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("[T]"));

    let mut encoder = Encoder::new();
    Option::<bool>::None.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("[]"));

    let mut encoder = Encoder::new();
    vec![1, 2, 3].as_slice().encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("[1,2,3]"));

    let mut encoder = Encoder::new();
    vec![1, 2, 3]
        .into_boxed_slice()
        .encode_using(&mut encoder)
        .unwrap();
    assert_eq!(encoder.as_str(), Ok("[1,2,3]"));

    let mut encoder = Encoder::new();
    vec![1, 2, 3].encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("[1,2,3]"));
}
