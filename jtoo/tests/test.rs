use jtoo::{Pack, PackError, Packer};

#[derive(Eq, PartialEq /*Pack*/)]
struct Message {
    bool1: bool,
}
impl Pack for Message {
    fn pack(&self) -> Result<String, PackError> {
        let mut packer = Packer::new();
        packer.open_list()?;
        packer.open_list()?;
        packer.open_string()?;
        packer.append_string("bool1")?;
        packer.close_string()?;
        packer.append_bool(self.bool1)?;
        packer.close_list()?;
        packer.close_list()?;
        packer.to_string()
    }
}
#[test]
fn pack_ok() {
    assert_eq!(
        Message { bool1: true }.pack(),
        Ok("[[\"bool1\",T]]".to_string())
    );
    assert_eq!(
        Message { bool1: false }.pack(),
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
