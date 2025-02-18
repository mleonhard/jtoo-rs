use jtoo::{Encode, EncodeError, Encoder};

#[test]
fn encode() {
    assert_eq!("abc".encode().unwrap().as_str(), r#""abc""#);
    assert_eq!("abc".to_string().encode().unwrap().as_str(), r#""abc""#);
    assert_eq!(
        "abc"
            .to_string()
            .into_boxed_str()
            .encode()
            .unwrap()
            .as_str(),
        r#""abc""#
    );
}

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.append_string("").unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""""#));

    let mut encoder = Encoder::new();
    encoder.append_string("string1").unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""string1""#));
}

#[test]
fn escaped() {
    let mut encoder = Encoder::new();
    encoder.append_string("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f").unwrap();
    assert_eq!(
        encoder.as_str(),
        Ok(
            r#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#
        )
    );
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_string("a"), Err(EncodeError::UnclosedString));
}

#[test]
fn in_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_string("a").unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#"["a",T]"#));
}
