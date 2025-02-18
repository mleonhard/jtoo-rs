use jtoo::{EncodeError, Encoder};

#[test]
fn empty() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""""#));
}

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_open_string("string1").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""string1""#));
}

#[test]
fn append_twice() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_open_string("a").unwrap();
    encoder.append_open_string("b").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#""ab""#));
}

#[test]
fn escaped() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    encoder.append_open_string("\x00 \x01 \x02 \x03 \x04 \x05 \x06 \x07 \x08 \x09 \x0a \x0b \x0c \x0d \x0e \x0f \x10 \x11 \x12 \x13 \x14 \x15 \x16 \x17 \x18 \x19 \x1a \x1b \x1c \x1d \x1e \x1f \" \\ \x7f").unwrap();
    encoder.close_string().unwrap();
    assert_eq!(
        encoder.as_str(),
        Ok(
            r#""\00 \01 \02 \03 \04 \05 \06 \07 \08 \09 \0a \0b \0c \0d \0e \0f \10 \11 \12 \13 \14 \15 \16 \17 \18 \19 \1a \1b \1c \1d \1e \1f \22 \5c \7f""#
        )
    );
}

#[test]
fn unclosed() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.append_bool(true), Err(EncodeError::UnclosedString));
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedString));
}

#[test]
fn nested() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_string(), Err(EncodeError::UnclosedString));
}

#[test]
fn closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    encoder.close_string().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#"["",T]"#));
}

#[test]
fn not_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(
        encoder.append_open_string("x"),
        Err(EncodeError::NotInString)
    );
    assert_eq!(encoder.close_string(), Err(EncodeError::NotInString));
}
