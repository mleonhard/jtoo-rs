use jtoo::{Encode, Encoder};

#[test]
fn encode_using() {
    let mut encoder = Encoder::new();
    true.encode_using(&mut encoder).unwrap();
    assert_eq!(encoder.as_str(), Ok("T"));
}
