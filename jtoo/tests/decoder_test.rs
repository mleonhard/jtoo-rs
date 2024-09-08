use jtoo::{escape_ascii, DecodeError, Decoder};

#[test]
fn empty() {
    let decoder = Decoder::new(b"");
    decoder.close().unwrap();
}

#[test]
fn consume_bool() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedBool(vec![]))),
        (b"\"a\"", Err(DecodeError::ExpectedBool(b"\"a\"".to_vec()))),
        (b"!", Err(DecodeError::ExpectedBool(b"!".to_vec()))),
        (b"T", Ok(true)),
        (b"F", Ok(false)),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(
            decoder.consume_bool(),
            expected,
            "bytes={bytes:?} expected={expected:?}"
        );
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn consume_string() {
    for (bytes, expected) in [
        (b"".as_slice(), Err(DecodeError::ExpectedString(vec![]))),
        (b"\"", Err(DecodeError::UnclosedString(b"\"".to_vec()))),
        (
            b"\"abc",
            Err(DecodeError::UnclosedString(b"\"abc".to_vec())),
        ),
        (b"\"abc\"", Ok("abc".to_string())),
        (
            &[b'"', 0xe4, 0xbd, b'"'],
            Err(DecodeError::NotUtf8(b"\"\xe4\xbd\"".to_vec())),
        ),
        (&[b'"', 0xe4, 0xbd, 0xa0, b'"'], Ok("你".to_string())),
    ] {
        let msg = format!("bytes=b\"{}\" expected={expected:?}", escape_ascii(bytes));
        let mut decoder = Decoder::new(bytes);
        assert_eq!(
            decoder.consume_string(),
            expected,
            "bytes={bytes:?} expected={expected:?}"
        );
        if expected.is_ok() {
            decoder.close().expect(&msg);
        }
    }
}

#[test]
fn list_missing_separator() {
    let mut decoder = Decoder::new(b"[TT]");
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_bool(),
        Err(DecodeError::ExpectedListSeparator(b"T]".to_vec()))
    );
}

#[test]
fn list_list_missing_separator() {
    let mut decoder = Decoder::new(b"[[][]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(
        decoder.consume_close_list(),
        Err(DecodeError::ExpectedListSeparator(b"[]]".to_vec()))
    );
}

#[test]
fn list_nested() {
    let mut decoder = Decoder::new(b"[[],[],[[T]]]");
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_open_list().unwrap();
    decoder.consume_open_list().unwrap();
    assert_eq!(decoder.consume_bool(), Ok(true));
    decoder.consume_close_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.consume_close_list().unwrap();
    decoder.close().unwrap();
}

#[test]
fn list_string_string() {
    let mut decoder = Decoder::new(b"[\"a\",\"b\"]");
    decoder.consume_open_list().unwrap();
    assert_eq!(decoder.consume_string(), Ok("a".to_string()));
    assert_eq!(decoder.consume_string(), Ok("b".to_string()));
    decoder.consume_close_list().unwrap();
    decoder.close().unwrap();
}
