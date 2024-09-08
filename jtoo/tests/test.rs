use jtoo::{Encode, EncodeError, Encoder};

#[derive(Eq, PartialEq /*Pack*/)]
struct Message {
    bool1: bool,
}
impl Encode for Message {
    fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
        encoder.open_list()?;
        encoder.open_list()?;
        encoder.open_string()?;
        encoder.append_string("bool1")?;
        encoder.close_string()?;
        encoder.append_bool(self.bool1)?;
        encoder.close_list()?;
        encoder.close_list()
    }
}
#[test]
fn pack_ok() {
    assert_eq!(
        Message { bool1: true }.encode(),
        Ok("[[\"bool1\",T]]".to_string())
    );
    assert_eq!(
        Message { bool1: false }.encode(),
        Ok("[[\"bool1\",F]]".to_string())
    );
}

// #[test]
// fn unpack_ok() {
//     #[derive(Eq, PartialEq, Unpack)]
//     struct Message {
//         bool1: bool,
//     }
//     assert_eq!(
//         Ok(Message { bool1: true }),
//         Message::unpack("[[\"bool1\",T]]")
//     );
// }
