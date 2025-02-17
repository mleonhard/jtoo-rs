use jtoo_derive_impl::derive_encode;
use quote::quote;

#[test]
fn empty() {
    let actual = derive_encode(quote! {
        struct Struct0 {}
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
        struct Struct0 {
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
                encoder.close_list()?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn all_field_types() {
    let actual = derive_encode(quote! {
        struct Struct0 {
            pub field0: bool,
            pub field1: i8,
            pub field2: u8,
            pub field3: i16,
            pub field4: u16,
            pub field5: i32,
            pub field6: u32,
            pub field7: i64,
            pub field8: u64,
            pub field9: &'static str,
            pub field10: Box<str>,
            pub field11: String,
            pub field12: &'static [u8],
            pub field13: Box<[u8]>,
            pub field14: Vec<u8>,
            pub field15: Option<bool>,
            pub field16: &'static [bool],
            pub field17: Box<[bool]>,
            pub field18: Vec<bool>,
            pub field19: Struct1,
            pub field20: Struct2<T>,
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
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field1")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field1, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field2")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field2, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field3")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field3, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field4")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field4, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field5")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field5, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field6")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field6, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field7")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field7, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field8")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field8, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field9")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field9, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field10")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field10, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field11")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field11, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field12")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field12, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field13")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field13, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field14")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field14, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field15")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field15, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field16")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field16, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field17")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field17, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field18")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field18, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field19")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field19, encoder)?;
                encoder.close_list()?;
                encoder.open_list()?;
                encoder.open_string()?;
                encoder.append_string("field20")?;
                encoder.close_string()?;
                jtoo::Encode::encode_using(&self.field20, encoder)?;
                encoder.close_list()?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn unnamed_fields() {
    let actual = derive_encode(quote! {
        struct Struct0(bool, String);
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Encode for Struct0 {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                jtoo::Encode::encode_using(&self.0, encoder)?;
                jtoo::Encode::encode_using(&self.1, encoder)?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn unit() {
    let actual = derive_encode(quote! {
        struct Struct0;
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
fn parameter() {
    let actual = derive_encode(quote! {
        struct Struct0<T0>(T0);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: jtoo::Encode> jtoo::Encode for Struct0<T0> {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                jtoo::Encode::encode_using(&self.0, encoder)?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn constrained_parameter() {
    let actual = derive_encode(quote! {
        struct Struct0<T0: Clone>(T0);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Clone + jtoo::Encode> jtoo::Encode for Struct0<T0> {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                jtoo::Encode::encode_using(&self.0, encoder)?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn two_parameters() {
    let actual = derive_encode(quote! {
        struct Struct0<T0: Sized + Clone + Send, T1>(T0, T1);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Sized + Clone + Send + jtoo::Encode, T1: jtoo::Encode> jtoo::Encode for Struct0<T0, T1> {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                encoder.open_list()?;
                jtoo::Encode::encode_using(&self.0, encoder)?;
                jtoo::Encode::encode_using(&self.1, encoder)?;
                encoder.close_list()
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}
