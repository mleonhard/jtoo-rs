use jtoo_derive_impl::derive_encode;
use quote::quote;

#[test]
fn empty() {
    let actual = derive_encode(quote! {
        pub struct Struct0 {}
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Encode for Struct0 {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn named_field() {
    let actual = derive_encode(quote! {
        pub struct Struct0 {
            pub field0: bool,
        }
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Encode for Struct0 {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field0")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field0, encoder)?;
                encoder.close_list()
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

// #[test]
// fn all_fields() {
//     let actual = derive_encode(quote! {
//         pub struct Struct0 {
//             pub field0: bool,
//             pub field1: i8,
//             pub field2: u8,
//             pub field3: i16,
//             pub field3: u16,
//             pub field4: i32,
//             pub field5: u32,
//             pub field6: i64,
//             pub field7: u64,
//             pub field8: &static str,
//             pub field9: Box<str>,
//             pub field10: String,
//             pub field11: &static [u8],
//             pub field12: Box<[u8]>,
//             pub field13: Vec<u8>,
//             pub field14: Option<bool>,
//             pub field15: &static [bool],
//             pub field16: Box<[bool]>,
//             pub field17: Vec<bool>,
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Struct0 {
//             fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
//                 encoder.open_list()?;
//                 jtoo::Encode::encode_using(&self.field0, encoder)?;
//                 jtoo::Encode::encode_using(&self.field1, encoder)?;
//                 jtoo::Encode::encode_using(&self.field2, encoder)?;
//                 jtoo::Encode::encode_using(&self.field3, encoder)?;
//                 jtoo::Encode::encode_using(&self.field4, encoder)?;
//                 jtoo::Encode::encode_using(&self.field5, encoder)?;
//                 jtoo::Encode::encode_using(&self.field6, encoder)?;
//                 jtoo::Encode::encode_using(&self.field7, encoder)?;
//                 jtoo::Encode::encode_using(&self.field8, encoder)?;
//                 jtoo::Encode::encode_using(&self.field9, encoder)?;
//                 jtoo::Encode::encode_using(&self.field10, encoder)?;
//                 jtoo::Encode::encode_using(&self.field11, encoder)?;
//                 jtoo::Encode::encode_using(&self.field12, encoder)?;
//                 jtoo::Encode::encode_using(&self.field13, encoder)?;
//                 jtoo::Encode::encode_using(&self.field14, encoder)?;
//                 jtoo::Encode::encode_using(&self.field15, encoder)?;
//                 jtoo::Encode::encode_using(&self.field16, encoder)?;
//                 jtoo::Encode::encode_using(&self.field17, encoder)?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
