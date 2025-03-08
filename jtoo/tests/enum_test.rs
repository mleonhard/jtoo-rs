use jtoo::{Decode, Encode, ErrorReason};

#[test]
fn unknown_variant() {
    #[derive(Decode, Debug)]
    enum Enum0 {
        Variant0,
    }
    assert_eq!(
        Enum0::decode(br#"["Unknown0"]"#).unwrap_err().reason,
        ErrorReason::UnknownEnumVariant
    );
}

#[test]
fn unit() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0,
    }
    let value = Enum0::Variant0;
    // TODO: Encode unit enums as plain strings, not string in list.
    assert_eq!(value.encode().unwrap().as_str(), r#"["Variant0"]"#);
    assert_eq!(Enum0::decode(br#"["Variant0"]"#).unwrap(), value);
    assert_eq!(
        Enum0::decode(br#"["Variant0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
}

#[test]
fn empty_tuple() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0(),
    }
    let value = Enum0::Variant0();
    assert_eq!(value.encode().unwrap().as_str(), r#"["Variant0"]"#);
    assert_eq!(Enum0::decode(br#"["Variant0"]"#).unwrap(), value);
    assert_eq!(
        Enum0::decode(br#"["Variant0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
}

#[test]
fn empty_named() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0 {},
    }
    let value = Enum0::Variant0 {};
    assert_eq!(value.encode().unwrap().as_str(), r#"["Variant0"]"#);
    assert_eq!(Enum0::decode(br#"["Variant0"]"#).unwrap(), value);
    assert_eq!(
        Enum0::decode(br#"["Variant0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
}

#[test]
fn named() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0 { field0: bool },
    }
    const STRING: &str = r#"["Variant0",["field0",T]]"#;
    let value = Enum0::Variant0 { field0: true };
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Enum0::decode(STRING.as_bytes()).unwrap(), value);
    assert_eq!(
        Enum0::decode(br#"["Variant0"]"#).unwrap_err().reason,
        ErrorReason::MissingField
    );
    assert_eq!(
        Enum0::decode(br#"["Variant0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedList
    );
    assert_eq!(
        Enum0::decode(br#"["Variant0",["field0",T],["unknown_field1",1]]"#)
            .unwrap_err()
            .reason,
        ErrorReason::UnknownField
    );
}

#[test]
fn many_fields() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    #[allow(clippy::struct_field_names)]
    enum Enum0 {
        Variant0 {
            field_a: bool,
            field_b: i8,
            field_c: u8,
            field_d: i16,
            field_e: u16,
            field_f: i32,
            field_g: u32,
            field_h: i64,
            field_i: u64,
            // field_j: &'static str,
            field_k: Box<str>,
            field_l: String,
            // field_m: &'static [u8],
            field_n: Box<[u8]>,
            field_o: Vec<u8>,
            field_p: Option<bool>,
            // field_q: &'static [bool],
            field_r: Box<[bool]>,
            field_s: Vec<bool>,
        },
    }
    const STRING: &str = r#"["Variant0",["field_a",T],["field_b",-1],["field_c",1],["field_d",-2],["field_e",2],["field_f",-3],["field_g",3],["field_h",-4],["field_i",4],["field_k","6"],["field_l","7"],["field_n",[10,11]],["field_o",[12,13]],["field_p",[T]],["field_r",[T,F]],["field_s",[T,T,F]]]"#;
    let value = Enum0::Variant0 {
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
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Enum0::decode(STRING.as_bytes()).unwrap(), value);
}

#[test]
fn encode_reference_fields() {
    #[derive(Encode)]
    #[allow(clippy::struct_field_names)]
    enum Enum0 {
        Variant0 {
            field_a: &'static str,
            field_b: &'static [u8],
            field_c: &'static [bool],
        },
    }
    let value = Enum0::Variant0 {
        field_a: "1",
        field_b: &[2, 3],
        field_c: &[false, true],
    };
    assert_eq!(
        value.encode(),
        Ok(r#"["Variant0",["field_a","1"],["field_b",[2,3]],["field_c",[F,T]]]"#.to_string())
    );
}

#[test]
fn tuple() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0(bool, String, u8),
    }
    const STRING: &str = r#"["Variant0",T,"value0",2]"#;
    let value = Enum0::Variant0(true, String::from("value0"), 2);
    assert_eq!(value.encode().unwrap(), STRING);
    assert_eq!(Enum0::decode(STRING.as_bytes()).unwrap(), value);
    assert_eq!(
        Enum0::decode(br#"["Variant0",T,2]"#).unwrap_err().reason,
        ErrorReason::ExpectedString
    );
    assert_eq!(
        Enum0::decode(br#"["Variant0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedString
    );
    assert_eq!(
        Enum0::decode(br#"["Variant0",T,"a",1,99]"#)
            .unwrap_err()
            .reason,
        ErrorReason::ExpectedListEnd
    );
}

#[test]
fn nested() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Variant0 { field_a: bool },
    }
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum1 {
        Variant1(bool),
    }
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum2 {
        Variant2,
    }
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum3 {
        Variant3 {
            field_b: Enum0,
            field_c: Enum1,
            field_d: Enum2,
        },
    }
    const STRING: &str = r#"["Variant3",["field_b",["Variant0",["field_a",T]]],["field_c",["Variant1",F]],["field_d",["Variant2"]]]"#;
    let value = Enum3::Variant3 {
        field_b: Enum0::Variant0 { field_a: true },
        field_c: Enum1::Variant1(false),
        field_d: Enum2::Variant2,
    };
    assert_eq!(value.encode().unwrap().as_str(), STRING);
    assert_eq!(Enum3::decode(STRING.as_bytes()).unwrap(), value);
}

#[test]
fn parameterized() {
    #[derive(Decode, Encode, Debug, Eq, PartialEq)]
    enum Enum0<T: Sized + Clone + Send> {
        Variant0(T),
    }
    let value = Enum0::Variant0(true);
    assert_eq!(value.encode(), Ok(r#"["Variant0",T]"#.to_string()));

    let value: Enum0<u8> = Decode::decode(br#"["Variant0",7]"#).unwrap();
    assert_eq!(value, Enum0::Variant0(7));
}
