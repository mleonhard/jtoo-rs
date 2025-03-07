use jtoo::{Decode, Encode, ErrorReason};
use jtoo_derive::{Decode, Encode};

#[test]
fn encode() {
    #[derive(Encode, PartialEq, Debug)]
    enum Enum0 {
        Unit0,
        Tuple0(bool),
        Tuple1(bool, u8),
        Named0 { field0: bool },
        Named1 { field0: bool, field1: u8 },
    }
    assert_eq!(Enum0::Unit0.encode().unwrap().as_str(), r#"["Unit0"]"#);
    assert_eq!(
        Enum0::Tuple0(true).encode().unwrap().as_str(),
        r#"["Tuple0",T]"#
    );
    assert_eq!(
        Enum0::Tuple1(false, 2).encode().unwrap().as_str(),
        r#"["Tuple1",F,2]"#
    );
    assert_eq!(
        Enum0::Named0 { field0: true }.encode().unwrap().as_str(),
        r#"["Named0",["field0",T]]"#
    );
    assert_eq!(
        Enum0::Named1 {
            field0: false,
            field1: 2
        }
        .encode()
        .unwrap()
        .as_str(),
        r#"["Named1",["field0",F],["field1",2]]"#
    );
}

#[test]
fn decode() {
    #[derive(Decode, Debug, Eq, PartialEq)]
    enum Enum0 {
        Unit0,
        Tuple0(bool),
        Tuple1(bool, u8),
        Named0 { field0: bool },
        Named1 { field0: bool, field1: u8 },
    }
    assert_eq!(Enum0::decode(br#"["Unit0"]"#).unwrap(), Enum0::Unit0);
    assert_eq!(
        Enum0::decode(br#"["Tuple0",F]"#).unwrap(),
        Enum0::Tuple0(false)
    );
    assert_eq!(
        Enum0::decode(br#"["Tuple0"]"#).unwrap_err().reason,
        ErrorReason::ExpectedBool
    );
    assert_eq!(
        Enum0::decode(br#"["Tuple0",T,T]"#).unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
    assert_eq!(
        Enum0::decode(br#"["Tuple1",T,7]"#).unwrap(),
        Enum0::Tuple1(true, 7)
    );
    assert_eq!(
        Enum0::decode(br#"["Tuple1",T,7]"#).unwrap(),
        Enum0::Tuple1(true, 7)
    );
    assert_eq!(
        Enum0::decode(br#"["Named0",["field0",F]]"#).unwrap(),
        Enum0::Named0 { field0: false }
    );
    assert_eq!(
        Enum0::decode(br#"["Named1",["field1",7],["field0",F]]"#).unwrap(),
        Enum0::Named1 {
            field0: false,
            field1: 7
        }
    );
    assert_eq!(
        Enum0::decode(br#"["Named0",T]"#).unwrap_err().reason,
        ErrorReason::ExpectedList
    );
    assert_eq!(
        Enum0::decode(br#"["Named0"]"#).unwrap_err().reason,
        ErrorReason::MissingField
    );
    assert_eq!(
        Enum0::decode(br#"["Unknown0"]"#).unwrap_err().reason,
        ErrorReason::UnknownEnumVariant
    );
}

#[test]
fn parameterized() {
    #[derive(Encode)]
    enum Enum0<T: Sized + Clone + Send> {
        Tuple0(T),
    }
    let value = Enum0::Tuple0(true);
    assert_eq!(value.encode(), Ok(r#"["Tuple0",T]"#.to_string()));
}
