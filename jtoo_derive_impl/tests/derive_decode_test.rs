use jtoo_derive_impl::derive_decode;
use quote::quote;

#[test]
fn struct_unit() {
    let actual = derive_decode(quote! {
        struct Struct0;
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Encode for Struct0 {
            fn decode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
                encoder.open_list()?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

// #[test]
// fn struct_named_field() {
//     let actual = derive_encode(quote! {
//         struct Struct0 {
//             pub field0: bool,
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Struct0 {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field0")?;
//                 jtoo::Encode::encode_using(&self.field0, encoder)?;
//                 encoder.close_list()?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[allow(clippy::too_many_lines)]
// #[test]
// fn struct_all_field_types() {
//     let actual = derive_encode(quote! {
//         struct Struct0 {
//             pub field0: bool,
//             pub field1: i8,
//             pub field2: u8,
//             pub field3: i16,
//             pub field4: u16,
//             pub field5: i32,
//             pub field6: u32,
//             pub field7: i64,
//             pub field8: u64,
//             pub field9: &'static str,
//             pub field10: Box<str>,
//             pub field11: String,
//             pub field12: &'static [u8],
//             pub field13: Box<[u8]>,
//             pub field14: Vec<u8>,
//             pub field15: Option<bool>,
//             pub field16: &'static [bool],
//             pub field17: Box<[bool]>,
//             pub field18: Vec<bool>,
//             pub field19: Struct1,
//             pub field20: Struct2<T>,
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Struct0 {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field0")?;
//                 jtoo::Encode::encode_using(&self.field0, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field1")?;
//                 jtoo::Encode::encode_using(&self.field1, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field2")?;
//                 jtoo::Encode::encode_using(&self.field2, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field3")?;
//                 jtoo::Encode::encode_using(&self.field3, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field4")?;
//                 jtoo::Encode::encode_using(&self.field4, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field5")?;
//                 jtoo::Encode::encode_using(&self.field5, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field6")?;
//                 jtoo::Encode::encode_using(&self.field6, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field7")?;
//                 jtoo::Encode::encode_using(&self.field7, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field8")?;
//                 jtoo::Encode::encode_using(&self.field8, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field9")?;
//                 jtoo::Encode::encode_using(&self.field9, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field10")?;
//                 jtoo::Encode::encode_using(&self.field10, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field11")?;
//                 jtoo::Encode::encode_using(&self.field11, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field12")?;
//                 jtoo::Encode::encode_using(&self.field12, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field13")?;
//                 jtoo::Encode::encode_using(&self.field13, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field14")?;
//                 jtoo::Encode::encode_using(&self.field14, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field15")?;
//                 jtoo::Encode::encode_using(&self.field15, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field16")?;
//                 jtoo::Encode::encode_using(&self.field16, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field17")?;
//                 jtoo::Encode::encode_using(&self.field17, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field18")?;
//                 jtoo::Encode::encode_using(&self.field18, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field19")?;
//                 jtoo::Encode::encode_using(&self.field19, encoder)?;
//                 encoder.close_list()?;
//                 encoder.open_list()?;
//                 encoder.append_string("field20")?;
//                 jtoo::Encode::encode_using(&self.field20, encoder)?;
//                 encoder.close_list()?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[test]
// fn struct_tuple() {
//     let actual = derive_encode(quote! {
//         struct Struct0(bool, String);
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Struct0 {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 jtoo::Encode::encode_using(&self.0, encoder)?;
//                 jtoo::Encode::encode_using(&self.1, encoder)?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[test]
// fn struct_parameter() {
//     let actual = derive_encode(quote! {
//         struct Struct0<T0>(T0);
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: jtoo::Encode> jtoo::Encode for Struct0<T0> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 jtoo::Encode::encode_using(&self.0, encoder)?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[test]
// fn struct_constrained_parameter() {
//     let actual = derive_encode(quote! {
//         struct Struct0<T0: Clone>(T0);
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: Clone + jtoo::Encode> jtoo::Encode for Struct0<T0> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 jtoo::Encode::encode_using(&self.0, encoder)?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[test]
// fn struct_two_parameters() {
//     let actual = derive_encode(quote! {
//         struct Struct0<T0: Sized + Clone + Send, T1>(T0, T1);
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: Sized + Clone + Send + jtoo::Encode, T1: jtoo::Encode> jtoo::Encode for Struct0<T0, T1> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 jtoo::Encode::encode_using(&self.0, encoder)?;
//                 jtoo::Encode::encode_using(&self.1, encoder)?;
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(expected.to_string(), actual.to_string());
// }
//
// #[test]
// fn enums() {
//     let actual = derive_encode(quote! {
//         enum Enum0 {
//             Unit0,
//             Tuple0(bool),
//             Tuple1(bool, u8),
//             Named0 { named_field0: bool },
//             Named1 { named_field0: bool, named_field1: u8 },
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Enum0 {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 match self {
//                     Enum0::Unit0 => {
//                         encoder.append_string("Unit0")?;
//                     }
//                     Enum0::Tuple0(field0) => {
//                         encoder.append_string("Tuple0")?;
//                         jtoo::Encode::encode_using(field0, encoder)?;
//                     }
//                     Enum0::Tuple1(field0, field1) => {
//                         encoder.append_string("Tuple1")?;
//                         jtoo::Encode::encode_using(field0, encoder)?;
//                         jtoo::Encode::encode_using(field1, encoder)?;
//                     }
//                     Enum0::Named0 { named_field0 } => {
//                         encoder.append_string("Named0")?;
//                         encoder.open_list()?;
//                         encoder.open_list()?;
//                         encoder.append_string("named_field0")?;
//                         jtoo::Encode::encode_using(named_field0, encoder)?;
//                         encoder.close_list()?;
//                         encoder.close_list()?;
//                     }
//                     Enum0::Named1 { named_field0, named_field1 } => {
//                         encoder.append_string("Named1")?;
//                         encoder.open_list()?;
//                         encoder.open_list()?;
//                         encoder.append_string("named_field0")?;
//                         jtoo::Encode::encode_using(named_field0, encoder)?;
//                         encoder.close_list()?;
//                         encoder.open_list()?;
//                         encoder.append_string("named_field1")?;
//                         jtoo::Encode::encode_using(named_field1, encoder)?;
//                         encoder.close_list()?;
//                         encoder.close_list()?;
//                     }
//                 }
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(actual.to_string(), expected.to_string());
// }
//
// #[test]
// fn enum_parameter() {
//     let actual = derive_encode(quote! {
//         enum Enum0<T0> {
//             Tuple0(T0),
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: jtoo::Encode> jtoo::Encode for Enum0<T0> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 match self {
//                     Enum0::Tuple0(field0) => {
//                         encoder.append_string("Tuple0")?;
//                         jtoo::Encode::encode_using(field0, encoder)?;
//                     }
//                 }
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(actual.to_string(), expected.to_string());
// }
//
// #[test]
// fn enum_constrained_parameter() {
//     let actual = derive_encode(quote! {
//         enum Enum0<T0: Sized + Clone + Send> {
//             Tuple0(T0),
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: Sized + Clone + Send + jtoo::Encode> jtoo::Encode for Enum0<T0> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 match self {
//                     Enum0::Tuple0(field0) => {
//                         encoder.append_string("Tuple0")?;
//                         jtoo::Encode::encode_using(field0, encoder)?;
//                     }
//                 }
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(actual.to_string(), expected.to_string());
// }
//
// #[test]
// fn enum_two_parameters() {
//     let actual = derive_encode(quote! {
//         enum Enum0<T0: Sized + Clone + Send, T1> {
//             Tuple0(T0, T1),
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl <T0: Sized + Clone + Send + jtoo::Encode, T1: jtoo::Encode> jtoo::Encode for Enum0<T0, T1> {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 encoder.open_list()?;
//                 match self {
//                     Enum0::Tuple0(field0, field1) => {
//                         encoder.append_string("Tuple0")?;
//                         jtoo::Encode::encode_using(field0, encoder)?;
//                         jtoo::Encode::encode_using(field1, encoder)?;
//                     }
//                 }
//                 encoder.close_list()
//             }
//         }
//     };
//     assert_eq!(actual.to_string(), expected.to_string());
// }
//
// #[test]
// fn unions() {
//     let actual = derive_encode(quote! {
//         #[repr(C)]
//         union Union0 {
//             field0: u8,
//         }
//     })
//     .unwrap();
//     let expected = quote! {
//         impl jtoo::Encode for Union0 {
//             fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
//                 compile_error!("This macro does not support union types.");
//             }
//         }
//     };
//     assert_eq!(actual.to_string(), expected.to_string());
// }
