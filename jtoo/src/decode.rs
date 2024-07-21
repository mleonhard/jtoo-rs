#[derive(Debug, Eq, PartialEq)]
pub enum DecodeError {}

pub trait Decode {
    fn decode(bytes: &[u8]) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub struct Decoder<'a> {
    bytes: &'a [u8],
}
impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
}
