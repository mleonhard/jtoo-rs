//! This crate implements the derive macros for the [`jtoo`](https://crates.io/crates/jtoo)
//! `Decode` and `Encode` traits.
//!
//! # Development
//!
//! Inspect macro expansion:
//! 1. Comment out tests except one
//! 2. `cargo expand --package jtoo --test struct_test |less`
#![forbid(unsafe_code)]
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, DataEnum, DataStruct, DeriveInput, Fields, GenericParam};

fn encode_using_function_body_for_struct(data: &DataStruct) -> TokenStream {
    let unit = quote! {
        encoder.open_list()?;
        encoder.close_list()
    };
    match &data.fields {
        Fields::Named(ref fields) if fields.named.is_empty() => unit,
        Fields::Named(ref fields) => {
            let per_field_calls = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_string = Literal::string(&field_name.to_string());
                quote_spanned! {field.span()=>
                    encoder.open_list()?;
                    encoder.append_string(#field_name_string)?;
                    jtoo::Encode::encode_using(&self.#field_name, encoder)?;
                    encoder.close_list()?;
                }
            });
            quote! {
                encoder.open_list()?;
                #(#per_field_calls)*
                encoder.close_list()
            }
        }
        Fields::Unnamed(ref fields) if fields.unnamed.is_empty() => unit,
        Fields::Unnamed(ref fields) => {
            let per_field_calls = fields.unnamed.iter().enumerate().map(|(n, field)| {
                let index = Literal::usize_unsuffixed(n);
                quote_spanned! {field.span()=>
                    jtoo::Encode::encode_using(&self . #index, encoder)?;
                }
            });
            quote! {
                encoder.open_list()?;
                #(#per_field_calls)*
                encoder.close_list()
            }
        }
        Fields::Unit => unit,
    }
}

fn encode_using_function_body_for_enum(enum_ident: &Ident, data: &DataEnum) -> TokenStream {
    let arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_literal = Literal::string(&variant_ident.to_string());
        match &variant.fields {
            Fields::Unit => quote_spanned! {variant.span()=>
                #enum_ident :: #variant_ident => {
                    encoder.append_string(#variant_literal)?;
                }
            },
            Fields::Named(fields) if fields.named.is_empty() => quote_spanned! {variant.span()=>
                #enum_ident :: #variant_ident { } => {
                    encoder.append_string(#variant_literal)?;
                }
            },
            Fields::Named(fields) => {
                let field_idents = fields
                    .named
                    .iter()
                    .map(|field| field.ident.as_ref().unwrap());
                let encoder_calls = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    let field_literal = Literal::string(&field_ident.to_string());
                    quote_spanned! {field.span()=>
                        encoder.open_list()?;
                        encoder.append_string(#field_literal)?;
                        jtoo::Encode::encode_using(#field_ident, encoder)?;
                        encoder.close_list()?;
                    }
                });
                quote_spanned! {variant.span()=>
                    #enum_ident :: #variant_ident { #(#field_idents ),* } => {
                        encoder.append_string(#variant_literal)?;
                        #(#encoder_calls)*
                    }
                }
            }
            Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                quote_spanned! {variant.span()=>
                    #enum_ident :: #variant_ident ( ) => {
                        encoder.append_string(#variant_literal)?;
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let field_idents = fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(n, field)| format_ident!("field{}", n, span = field.ident.span()));
                let encoder_calls = fields.unnamed.iter().enumerate().map(|(n, field)| {
                    let field_ident = format_ident!("field{}", n, span = field.ident.span());
                    quote_spanned! {field.span()=>
                        jtoo::Encode::encode_using(#field_ident, encoder)?;
                    }
                });
                quote_spanned! {variant.span()=>
                    #enum_ident :: #variant_ident ( #(#field_idents ),* ) => {
                        encoder.append_string(#variant_literal)?;
                        #(#encoder_calls)*
                    }
                }
            }
        }
    });
    quote! {
        encoder.open_list()?;
        match self {
            #(#arms)*
        }
        encoder.close_list()
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn derive_encode(stream: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(stream)?;

    // Add a bound `T: Encode` to every type parameter T.
    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(jtoo::Encode));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let encoder_calls = match &input.data {
        Data::Struct(data) => encode_using_function_body_for_struct(data),
        Data::Enum(data) => encode_using_function_body_for_enum(&input.ident, data),
        Data::Union(_) => {
            quote! { compile_error!("`Encode` macro does not support union types."); }
        }
    };
    let struct_name = input.ident;
    Ok(quote! {
        impl #impl_generics jtoo::Encode for #struct_name #ty_generics #where_clause {
            fn encode_using(&self, encoder: &mut jtoo::Encoder) -> Result<(), jtoo::EncodeError> {
                #encoder_calls
            }
        }
    })
}

fn read_named_fields(fields: &syn::FieldsNamed) -> TokenStream {
    let declare_opt_field_vars = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let opt_field_name_string = format!("opt_{field_name}");
        let opt_field_name = Ident::new(&opt_field_name_string, field.span());
        let field_type = field.ty.clone();
        quote_spanned! {field.span()=>
            let mut #opt_field_name: Option<#field_type> = None;
        }
    });
    let match_arms = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_name_string = Literal::string(&field_name.to_string());
        let opt_field_name_string = format!("opt_{field_name}");
        let opt_field_name = Ident::new(&opt_field_name_string, field.span());
        quote_spanned! {field.span()=>
            #field_name_string => {
                // TODO: Include the field name in the error message.
                let value = jtoo::Decode::decode_using(decoder)?;
                #opt_field_name = Some(value);
            }
        }
    });
    quote! {
        #(#declare_opt_field_vars)*
        while decoder.has_another_list_item() {
            decoder.consume_list_open()?;
            match decoder.consume_string()?.as_str() {
                #(#match_arms)*
                // TODO: Add an option to allow unknown fields.
                _ => return Err(decoder.err(jtoo::ErrorReason::UnknownField)),
            }
            decoder.consume_list_close()?;
        }
    }
}

fn assign_fields(fields: &syn::FieldsNamed) -> TokenStream {
    fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        //let field_name_string = Literal::string(&field_name.to_string());
        let opt_field_name_string = format!("opt_{field_name}");
        let opt_field_name = Ident::new(&opt_field_name_string, field.span());
        quote_spanned! {field.span()=>
            // TODO: Include the field name in the error message.
            #field_name : #opt_field_name .ok_or_else(|| decoder.err(jtoo::ErrorReason::MissingField))?,
        }
    }).collect()
}

fn decode_using_function_body_for_struct(data: &DataStruct) -> TokenStream {
    match &data.fields {
        Fields::Named(ref fields) if fields.named.is_empty() => {
            quote! {
                decoder.consume_list_open()?;
                decoder.consume_list_close()?;
                Ok(Self {})
            }
        }
        Fields::Named(ref fields) => {
            let read_fields_statements = read_named_fields(fields);
            let assign_fields_statements = assign_fields(fields);
            quote! {
                decoder.consume_list_open()?;
                #read_fields_statements
                let value = Self {
                    #assign_fields_statements
                };
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
        Fields::Unnamed(ref fields) if fields.unnamed.is_empty() => {
            quote! {
                decoder.consume_list_open()?;
                decoder.consume_list_close()?;
                Ok(Self())
            }
        }
        Fields::Unnamed(ref fields) => {
            let field_assignments = fields.unnamed.iter().map(|field| {
                quote_spanned! {field.span()=>
                    Decode::decode_using(decoder)?,
                }
            });
            quote! {
                decoder.consume_list_open()?;
                let value = Self(
                    #(#field_assignments)*
                );
                decoder.consume_list_close()?;
                Ok(value)
            }
        }
        Fields::Unit => {
            quote! {
                decoder.consume_list_open()?;
                decoder.consume_list_close()?;
                Ok(Self)
            }
        }
    }
}

fn decode_using_function_body_for_enum(enum_ident: &Ident, data: &DataEnum) -> TokenStream {
    let arms = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_literal = Literal::string(&variant_ident.to_string());
        match &variant.fields {
            Fields::Unit => quote_spanned! {variant.span()=>
                #variant_literal => #enum_ident :: #variant_ident ,
            },
            Fields::Named(fields) if fields.named.is_empty() => quote_spanned! {variant.span()=>
                #variant_literal => #enum_ident :: #variant_ident {},
            },
            Fields::Named(fields) => {
                let read_fields_statements = read_named_fields(fields);
                let assign_fields_statements = assign_fields(fields);
                quote_spanned! {variant.span()=>
                    #variant_literal => {
                        #read_fields_statements
                        #enum_ident :: #variant_ident {
                            #assign_fields_statements
                        }
                    }
                }
            }
            Fields::Unnamed(fields) if fields.unnamed.is_empty() => {
                quote_spanned! {variant.span()=>
                    #variant_literal => #enum_ident :: #variant_ident (),
                }
            }
            Fields::Unnamed(fields) => {
                let field_assignments = fields.unnamed.iter().map(|field| {
                    quote_spanned! {field.span()=>
                        Decode::decode_using(decoder)?,
                    }
                });
                quote! {
                    #variant_literal => #enum_ident :: #variant_ident (
                        #(#field_assignments)*
                    ),
                }
            }
        }
    });
    quote! {
        decoder.consume_list_open()?;
        let value = match decoder.consume_string()?.as_str() {
            #(#arms)*
            _ => return Err(decoder.err(jtoo::ErrorReason::UnknownEnumVariant)),
        };
        decoder.consume_list_close()?;
        Ok(value)
    }
}

#[allow(clippy::missing_errors_doc)]
pub fn derive_decode(stream: TokenStream) -> Result<TokenStream, syn::Error> {
    let input: DeriveInput = syn::parse2(stream)?;

    // Add a bound `T: Decode` to every type parameter T.
    let mut generics = input.generics;
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(jtoo::Decode));
        }
    }
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let decoder_calls = match &input.data {
        Data::Struct(data) => decode_using_function_body_for_struct(data),
        Data::Enum(data) => decode_using_function_body_for_enum(&input.ident, data),
        Data::Union(_) => {
            quote! { compile_error!("`Decode` macro does not support union types."); }
        }
    };
    let struct_name = input.ident;
    Ok(quote! {
        impl #impl_generics jtoo::Decode for #struct_name #ty_generics #where_clause {
            fn decode_using(decoder: &mut jtoo::Decoder) -> Result<Self, jtoo::DecodeError> {
                #decoder_calls
            }
        }
    })
}
