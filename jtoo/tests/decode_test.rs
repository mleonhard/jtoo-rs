use jtoo::{Decode, Decoder};

#[test]
fn decode_using() {
    assert_eq!(bool::decode_using(&mut Decoder::new(b"T")), Ok(true));
}
