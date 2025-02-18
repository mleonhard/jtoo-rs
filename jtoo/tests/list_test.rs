use jtoo::{Encode, EncodeError, Encoder};

#[test]
fn encode() {
    assert_eq!(Some(true).encode().unwrap().as_str(), "[T]");
    assert_eq!(Option::<bool>::None.encode().unwrap().as_str(), "[]");
    assert_eq!(vec![1, 2, 3].encode().unwrap().as_str(), "[1,2,3]");
    assert_eq!(
        vec![1, 2, 3].as_slice().encode().unwrap().as_str(),
        "[1,2,3]"
    );
    assert_eq!(
        vec![1, 2, 3].into_boxed_slice().encode().unwrap().as_str(),
        "[1,2,3]"
    );
}

#[test]
fn empty() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[]"));
}

#[test]
fn nested() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[[]]"));
}

#[test]
fn various_values() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.open_byte_string().unwrap();
    encoder.append_byte_string(&[0xa1]).unwrap();
    encoder.close_byte_string().unwrap();
    encoder.open_list().unwrap();
    encoder.append_bool(false).unwrap();
    encoder.close_list().unwrap();
    encoder.append_string("string1").unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok(r#"[T,Ba1,[F],"string1"]"#));
}

#[test]
fn unclosed_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_list(), Err(EncodeError::UnclosedString));
}

#[test]
fn closed() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[[],T]"));
}

#[test]
fn unclosed_list() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedList));
}

#[test]
fn not_in_list() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));
}

#[test]
fn not_in_list_nested() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));
}
