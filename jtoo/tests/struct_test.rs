use jtoo::{Decode, Encode, ErrorReason};
use jtoo_derive::{Decode, Encode};

#[test]
fn unit() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0;
    let value = Struct0;
    assert_eq!(value.encode().unwrap().as_str(), "[]");
    assert_eq!(Struct0::decode(b"[]").unwrap(), Struct0);
}

#[test]
fn empty_tuple() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0();
    let value = Struct0();
    assert_eq!(value.encode().unwrap().as_str(), "[]");
    assert_eq!(Struct0::decode(b"[]").unwrap(), Struct0());
}

#[test]
fn empty_named() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0 {}
    let value = Struct0 {};
    assert_eq!(value.encode().unwrap().as_str(), "[]");
    assert_eq!(Struct0::decode(b"[]").unwrap(), Struct0 {});
}

#[test]
fn named() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0 {
        pub field0: bool,
    }
    let value = Struct0 { field0: true };
    const STRING: &str = "[[\"field0\",T]]";
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Struct0::decode(STRING.as_bytes()).unwrap(), value);
    assert_eq!(
        Struct0::decode(b"[]").unwrap_err().reason,
        ErrorReason::MissingField
    );
}

#[test]
fn many_fields() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    #[allow(clippy::struct_field_names)]
    struct Struct0 {
        pub field_a: bool,
        pub field_b: i8,
        pub field_c: u8,
        pub field_d: i16,
        pub field_e: u16,
        pub field_f: i32,
        pub field_g: u32,
        pub field_h: i64,
        pub field_i: u64,
        // pub field_j: &'static str,
        pub field_k: Box<str>,
        pub field_l: String,
        // pub field_m: &'static [u8],
        pub field_n: Box<[u8]>,
        pub field_o: Vec<u8>,
        pub field_p: Option<bool>,
        // pub field_q: &'static [bool],
        pub field_r: Box<[bool]>,
        pub field_s: Vec<bool>,
    }
    let value = Struct0 {
        field_a: true,
        field_b: -1,
        field_c: 1,
        field_d: -2,
        field_e: 2,
        field_f: -3,
        field_g: 3,
        field_h: -4,
        field_i: 4,
        // field_j: "5",
        field_k: "6".to_string().into_boxed_str(),
        field_l: "7".to_string(),
        // field_m: &[8, 9],
        field_n: vec![10, 11].into_boxed_slice(),
        field_o: vec![12, 13],
        field_p: Some(true),
        // field_q: &[false, true],
        field_r: vec![true, false].into_boxed_slice(),
        field_s: vec![true, true, false],
    };
    const STRING: &str = r#"[["field_a",T],["field_b",-1],["field_c",1],["field_d",-2],["field_e",2],["field_f",-3],["field_g",3],["field_h",-4],["field_i",4],["field_k","6"],["field_l","7"],["field_n",[10,11]],["field_o",[12,13]],["field_p",[T]],["field_r",[T,F]],["field_s",[T,T,F]]]"#;
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Struct0::decode(STRING.as_bytes()).unwrap(), value);
}

#[test]
fn encode_reference_fields() {
    #[derive(Encode)]
    #[allow(clippy::struct_field_names)]
    struct Struct0 {
        pub field_a: &'static str,
        pub field_b: &'static [u8],
        pub field_c: &'static [bool],
    }
    let value = Struct0 {
        field_a: "1",
        field_b: &[2, 3],
        field_c: &[false, true],
    };
    assert_eq!(
        value.encode(),
        Ok(r#"[["field_a","1"],["field_b",[2,3]],["field_c",[F,T]]]"#.to_string())
    );
}

#[test]
fn tuple() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0(bool, String, u8);
    const STRING: &str = r#"[T,"value0",2]"#;
    let value = Struct0(true, String::from("value0"), 2);
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Struct0::decode(STRING.as_bytes()).unwrap(), value);
    assert_eq!(
        Struct0::decode(b"[T,2]").unwrap_err().reason,
        ErrorReason::ExpectedString
    );
    assert_eq!(
        Struct0::decode(b"[T]").unwrap_err().reason,
        ErrorReason::ExpectedString
    );
}

#[test]
fn nested() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0 {
        pub field_a: bool,
    }
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct1(bool);
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct2 {}
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct3 {
        pub field_b: Struct0,
        pub field_c: Struct1,
        pub field_d: Struct2,
    }
    let value = Struct3 {
        field_b: Struct0 { field_a: true },
        field_c: Struct1(false),
        field_d: Struct2 {},
    };
    const STRING: &str = r#"[["field_b",[["field_a",T]]],["field_c",[F]],["field_d",[]]]"#;
    assert_eq!(value.encode().unwrap().as_str(), STRING);
    assert_eq!(Struct3::decode(STRING.as_bytes()).unwrap(), value);
}

#[test]
fn parameterized() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    struct Struct0<T: Sized + Clone + Send>(T);
    let value = Struct0(true);
    assert_eq!(value.encode(), Ok("[T]".to_string()));

    let value: Struct0<u8> = Decode::decode(br#"[7]"#).unwrap();
    assert_eq!(value, Struct0(7));
}
