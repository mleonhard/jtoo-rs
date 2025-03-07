use jtoo::{Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

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
fn decode() {
    assert_eq!(Vec::<bool>::decode(b"[T]").unwrap(), vec![true]);
    assert_eq!(Vec::<u8>::decode(b"[1]").unwrap(), vec![1]);
    assert_eq!(
        Vec::<Vec<u8>>::decode(b"[[1],[2,3]]").unwrap(),
        vec![vec![1], vec![2, 3]]
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
fn close_list_not_in_list() {
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

#[test]
fn list_missing_separator() {
    let mut decoder = Decoder::new(b"[TT]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );
}

#[test]
fn list_nested() {
    let mut decoder = Decoder::new(b"[[],[],[[T]]]");
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_open().unwrap();
    decoder.consume_list_open().unwrap();
    assert_eq!(decoder.consume_bool(), Ok(true));
    decoder.consume_list_close().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_not_a_list() {
    assert_eq!(
        Decoder::new(b"T").consume_list_open().unwrap_err().reason,
        ErrorReason::ExpectedList
    );
}

#[test]
fn list_string_string() {
    let mut decoder = Decoder::new(b"[\"a\",\"b\"]");
    decoder.consume_list_open().unwrap();
    assert_eq!(decoder.consume_string(), Ok("a".to_string()));
    assert_eq!(decoder.consume_string(), Ok("b".to_string()));
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_has_next_item() {
    let mut decoder = Decoder::new(b"[T,T]");
    decoder.consume_list_open().unwrap();
    assert!(decoder.has_another_list_item());
    decoder.consume_bool().unwrap();
    assert!(decoder.has_another_list_item());
    decoder.consume_bool().unwrap();
    assert!(!decoder.has_another_list_item());
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();
}

#[test]
fn close_not_consumed() {
    let mut decoder = Decoder::new(b"[]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.close().unwrap_err().reason,
        ErrorReason::ListCloseNotConsumed
    );
}

#[test]
fn consume_list_close_not_in_list() {
    let mut decoder = Decoder::new(b"T");
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::NotInList
    );
}

#[test]
fn expected_list_end() {
    let mut decoder = Decoder::new(b"[");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
}
