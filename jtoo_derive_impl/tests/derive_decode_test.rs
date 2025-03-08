use jtoo_derive_impl::derive_decode;
use quote::quote;

#[test]
fn struct_unit() {
    let actual = derive_decode(quote! {
        struct Struct0;
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Struct0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                decoder.consume_list_close()?;
                Ok(Self)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn struct_named_field() {
    let actual = derive_decode(quote! {
        struct Struct0 {
            pub field0: bool,
        }
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Struct0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let mut opt_field0: Option<bool> = None;
                while decoder.has_another_list_item() {
                    decoder.consume_list_open()?;
                    match decoder.consume_string()?.as_str() {
                        "field0" => {
                            // TODO: Include the field name in the error message.
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field0 = Some(value);
                        }
                        // TODO: Add an option to allow unknown fields.
                        _ => return Err(decoder.err(jtoo::ErrorReason::UnknownField)),
                    }
                    decoder.consume_list_close()?;
                }
                let value = Self {
                    field0: opt_field0.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[allow(clippy::too_many_lines)]
#[test]
fn struct_all_field_types() {
    let actual = derive_decode(quote! {
        struct Struct0 {
            pub field_a: bool,
            pub field_b: i8,
            pub field_c: u8,
            pub field_d: i16,
            pub field_e: u16,
            pub field_f: i32,
            pub field_g: u32,
            pub field_h: i64,
            pub field_i: u64,
            // pub field_j: &'static str,
            pub field_k: Box<str>,
            pub field_l: String,
            // pub field_m: &'static [u8],
            pub field_n: Box<[u8]>,
            pub field_o: Vec<u8>,
            pub field_p: Option<bool>,
            // pub field_q: &'static [bool],
            pub field_r: Box<[bool]>,
            pub field_s: Vec<bool>,
        }
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Struct0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let mut opt_field_a: Option<bool> = None;
                let mut opt_field_b: Option<i8> = None;
                let mut opt_field_c: Option<u8> = None;
                let mut opt_field_d: Option<i16> = None;
                let mut opt_field_e: Option<u16> = None;
                let mut opt_field_f: Option<i32> = None;
                let mut opt_field_g: Option<u32> = None;
                let mut opt_field_h: Option<i64> = None;
                let mut opt_field_i: Option<u64> = None;
                let mut opt_field_k: Option<Box<str> > = None;
                let mut opt_field_l: Option<String> = None;
                let mut opt_field_n: Option<Box<[u8]> > = None;
                let mut opt_field_o: Option<Vec<u8> > = None;
                let mut opt_field_p: Option<Option<bool> > = None;
                let mut opt_field_r: Option<Box<[bool]> > = None;
                let mut opt_field_s: Option<Vec<bool> > = None;
                while decoder.has_another_list_item() {
                    decoder.consume_list_open()?;
                    match decoder.consume_string()?.as_str() {
                        "field_a" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_a = Some(value);
                        }
                        "field_b" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_b = Some(value);
                        }
                        "field_c" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_c = Some(value);
                        }
                        "field_d" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_d = Some(value);
                        }
                        "field_e" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_e = Some(value);
                        }
                        "field_f" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_f = Some(value);
                        }
                        "field_g" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_g = Some(value);
                        }
                        "field_h" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_h = Some(value);
                        }
                        "field_i" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_i = Some(value);
                        }
                        "field_k" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_k = Some(value);
                        }
                        "field_l" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_l = Some(value);
                        }
                        "field_n" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_n = Some(value);
                        }
                        "field_o" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_o = Some(value);
                        }
                        "field_p" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_p = Some(value);
                        }
                        "field_r" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_r = Some(value);
                        }
                        "field_s" => {
                            let value = jtoo::Decode::decode_using(decoder)?;
                            opt_field_s = Some(value);
                        }
                        _ => return Err(decoder.err(jtoo::ErrorReason::UnknownField)),
                    }
                    decoder.consume_list_close()?;
                }
                let value = Self {
                    field_a: opt_field_a.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_b: opt_field_b.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_c: opt_field_c.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_d: opt_field_d.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_e: opt_field_e.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_f: opt_field_f.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_g: opt_field_g.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_h: opt_field_h.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_i: opt_field_i.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_k: opt_field_k.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_l: opt_field_l.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_n: opt_field_n.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_o: opt_field_o.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_p: opt_field_p.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_r: opt_field_r.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                    field_s: opt_field_s.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn struct_tuple() {
    let actual = derive_decode(quote! {
        struct Struct0(bool, String);
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Struct0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = Self(
                    Decode::decode_using(decoder)?,
                    Decode::decode_using(decoder)?,
                );
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn struct_parameter() {
    let actual = derive_decode(quote! {
        struct Struct0<T0>(T0);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: jtoo::Decode> jtoo::Decode for Struct0<T0> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = Self(
                    Decode::decode_using(decoder)?,
                );
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn struct_constrained_parameter() {
    let actual = derive_decode(quote! {
        struct Struct0<T0: Clone>(T0);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Clone + jtoo::Decode> jtoo::Decode for Struct0<T0> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = Self(
                    Decode::decode_using(decoder)?,
                );
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn struct_two_parameters() {
    let actual = derive_decode(quote! {
        struct Struct0<T0: Sized + Clone + Send, T1>(T0, T1);
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Sized + Clone + Send + jtoo::Decode, T1: jtoo::Decode> jtoo::Decode for Struct0<T0, T1> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = Self(
                    Decode::decode_using(decoder)?,
                    Decode::decode_using(decoder)?,
                );
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(expected.to_string(), actual.to_string());
}

#[test]
fn enums() {
    let actual = derive_decode(quote! {
        enum Enum0 {
            Unit0,
            Tuple0(bool),
            Tuple1(bool, u8),
            Named0 { field0: bool },
            Named1 { field0: bool, field1: u8 },
        }
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Enum0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = match decoder.consume_string()?.as_str() {
                    "Unit0" => Enum0::Unit0,
                    "Tuple0" => Enum0::Tuple0(
                            Decode::decode_using(decoder)?,
                    ),
                    "Tuple1" => Enum0::Tuple1(
                            Decode::decode_using(decoder)?,
                            Decode::decode_using(decoder)?,
                    ),
                    "Named0" => {
                        let mut opt_field0: Option<bool> = None;
                        while decoder.has_another_list_item() {
                            decoder.consume_list_open()?;
                            match decoder.consume_string()?.as_str() {
                                "field0" => {
                                    let value = jtoo::Decode::decode_using(decoder)?;
                                    opt_field0 = Some(value);
                                }
                                _ => return Err(decoder.err(jtoo::ErrorReason::UnknownField)),
                            }
                            decoder.consume_list_close()?;
                        }
                        Enum0::Named0 {
                            field0: opt_field0.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                        }
                    }
                    "Named1" => {
                        let mut opt_field0: Option<bool> = None;
                        let mut opt_field1: Option<u8> = None;
                        while decoder.has_another_list_item() {
                            decoder.consume_list_open()?;
                            match decoder.consume_string()?.as_str() {
                                "field0" => {
                                    let value = jtoo::Decode::decode_using(decoder)?;
                                    opt_field0 = Some(value);
                                }
                                "field1" => {
                                    let value = jtoo::Decode::decode_using(decoder)?;
                                    opt_field1 = Some(value);
                                }
                                _ => return Err(decoder.err(jtoo::ErrorReason::UnknownField)),
                            }
                            decoder.consume_list_close()?;
                        }
                        Enum0::Named1 {
                            field0: opt_field0.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                            field1: opt_field1.ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
                        }
                    }
                    _ => return Err(decoder.err(jtoo::ErrorReason::UnknownEnumVariant)),
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn enum_parameter() {
    let actual = derive_decode(quote! {
        enum Enum0<T0> {
            Tuple0(T0),
        }
    })
    .unwrap();
    let expected = quote! {
        impl <T0: jtoo::Decode> jtoo::Decode for Enum0<T0> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = match decoder.consume_string()?.as_str() {
                    "Tuple0" => Enum0::Tuple0(
                            Decode::decode_using(decoder)?,
                    ),
                    _ => return Err(decoder.err(jtoo::ErrorReason::UnknownEnumVariant)),
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn enum_constrained_parameter() {
    let actual = derive_decode(quote! {
        enum Enum0<T0: Sized + Clone + Send> {
            Tuple0(T0),
        }
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Sized + Clone + Send + jtoo::Decode> jtoo::Decode for Enum0<T0> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = match decoder.consume_string()?.as_str() {
                    "Tuple0" => Enum0::Tuple0(
                            Decode::decode_using(decoder)?,
                    ),
                    _ => return Err(decoder.err(jtoo::ErrorReason::UnknownEnumVariant)),
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn enum_two_parameters() {
    let actual = derive_decode(quote! {
        enum Enum0<T0: Sized + Clone + Send, T1> {
            Tuple0(T0, T1),
        }
    })
    .unwrap();
    let expected = quote! {
        impl <T0: Sized + Clone + Send + jtoo::Decode, T1: jtoo::Decode> jtoo::Decode for Enum0<T0, T1> {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                decoder.consume_list_open()?;
                let value = match decoder.consume_string()?.as_str() {
                    "Tuple0" => Enum0::Tuple0(
                            Decode::decode_using(decoder)?,
                            Decode::decode_using(decoder)?,
                    ),
                    _ => return Err(decoder.err(jtoo::ErrorReason::UnknownEnumVariant)),
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}

#[test]
fn unions() {
    let actual = derive_decode(quote! {
        #[repr(C)]
        union Union0 {
            field0: u8,
        }
    })
    .unwrap();
    let expected = quote! {
        impl jtoo::Decode for Union0 {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                compile_error!("`Decode` macro does not support union types.");
            }
        }
    };
    assert_eq!(actual.to_string(), expected.to_string());
}
