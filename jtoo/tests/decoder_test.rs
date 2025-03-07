use jtoo::{Decoder, ErrorReason};

#[test]
fn empty() {
    let decoder = Decoder::new(b"");
    decoder.close().unwrap();
}

#[test]
fn close_data_not_consumed() {
    assert_eq!(
        Decoder::new(b"Y").close().unwrap_err().reason,
        ErrorReason::DataNotConsumed
    );
}

#[test]
fn debug() {
    let _ = format!("{:?}", Decoder::new(b"x"));
}
