use jtoo::{Decode, Decoder, Encode, EncodeError, Encoder, ErrorReason};

#[test]
fn append() {
    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[]"));

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

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[[]]"));

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_list().unwrap();
    encoder.close_list().unwrap();
    encoder.append_bool(true).unwrap();
    encoder.close_list().unwrap();
    assert_eq!(encoder.as_str(), Ok("[[],T]"));

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    assert_eq!(encoder.as_str(), Err(EncodeError::UnclosedList));

    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));

    let mut encoder = Encoder::new();
    encoder.open_list().unwrap();
    encoder.open_string().unwrap();
    assert_eq!(encoder.close_list(), Err(EncodeError::NotInList));
}

#[test]
fn append_in_string() {
    let mut encoder = Encoder::new();
    encoder.open_string().unwrap();
    assert_eq!(encoder.open_list(), Err(EncodeError::UnclosedString));
}

#[test]
fn consume() {
    let mut decoder = Decoder::new(b"T");
    assert_eq!(
        decoder.consume_list_open().unwrap_err().reason,
        ErrorReason::ExpectedList
    );
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::NotInList
    );

    let mut decoder = Decoder::new(b"[");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );

    let mut decoder = Decoder::new(b"[]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.close().unwrap_err().reason,
        ErrorReason::ListCloseNotConsumed
    );

    let mut decoder = Decoder::new(b"[]");
    decoder.consume_list_open().unwrap();
    assert!(!decoder.has_another_list_item());
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();

    let mut decoder = Decoder::new(b"[T]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_list_close().unwrap_err().reason,
        ErrorReason::ExpectedListEnd
    );
    assert!(decoder.has_another_list_item());
    decoder.consume_bool().unwrap();
    assert!(!decoder.has_another_list_item());
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();

    let mut decoder = Decoder::new(b"[TT]");
    decoder.consume_list_open().unwrap();
    assert_eq!(
        decoder.consume_bool().unwrap_err().reason,
        ErrorReason::ExpectedListSeparator
    );

    let mut decoder = Decoder::new(b"[1,2]");
    decoder.consume_list_open().unwrap();
    assert!(decoder.has_another_list_item());
    assert_eq!(decoder.consume_integer().unwrap(), 1);
    assert!(decoder.has_another_list_item());
    assert_eq!(decoder.consume_integer().unwrap(), 2);
    assert!(!decoder.has_another_list_item());
    decoder.consume_list_close().unwrap();
    decoder.close().unwrap();

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
fn decode() {
    assert_eq!(Vec::<bool>::decode(b"[T]").unwrap(), vec![true]);
    assert_eq!(Vec::<u8>::decode(b"[1]").unwrap(), vec![1]);
    assert_eq!(
        Vec::<Vec<u8>>::decode(b"[[1],[2,3]]").unwrap(),
        vec![vec![1], vec![2, 3]]
    );
}

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
