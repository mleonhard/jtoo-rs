//! This crate implements the derive macros for the [`jtoo`](https://crates.io/crates/jtoo)
//! `Encode` and `Decode` traits.
#![forbid(unsafe_code)]
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam};

#[macro_export]
macro_rules! dprintln {
    // ($($args:tt)+) => { println!( $($args)+ ) };
    ($($args:tt)+) => {};
}

/// Converts the bytes into an ASCII string.
#[allow(clippy::missing_panics_doc)]
pub fn escape_ascii(input: impl AsRef<[u8]>) -> String {
    let mut result = String::new();
    for byte in input.as_ref() {
        for ascii_byte in core::ascii::escape_default(*byte) {
            result.push_str(core::str::from_utf8(&[ascii_byte]).unwrap());
        }
    }
    result
}

/// # Errors
/// Returns `Err(String)` with a human-readable description of the problem.
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
    let Data::Struct(ref data) = &input.data else {
        unimplemented!()
    };

    let encoder_calls = match data.fields {
        Fields::Named(ref fields) => {
            let per_field_calls = fields.named.iter().map(|field| {
                let field_name = field.ident.as_ref().unwrap();
                let field_name_string = proc_macro2::Literal::string(&field_name.to_string());
                quote_spanned! {field.span()=>
                    encoder.open_list()?;
                    encoder.open_string()?;
                    encoder.append_string(#field_name_string)?;
                    encoder.close_string()?;
                    jtoo::Encode::encode_using(&self.#field_name, encoder)?;
                    encoder.close_list()
                }
            });
            quote! {
                encoder.open_list()?;
                #(#per_field_calls)*
                encoder.close_list()
            }
        }
        Fields::Unnamed(ref fields) => {
            let per_field_calls = fields.unnamed.iter().enumerate().map(|(n, field)| {
                quote_spanned! {field.span()=>
                    jtoo::Encode::encode_using(&self.#n, encoder)?;
                }
            });
            quote! {
                encoder.open_list()?;
                #(#per_field_calls)*
                encoder.close_list()
            }
        }
        Fields::Unit => {
            quote! {
                encoder.open_list()?;
                encoder.close_list()
            }
        }
    };
    let struct_name = input.ident;
    Ok(quote! {
        impl #impl_generics jtoo::Encode for #struct_name #ty_generics #where_clause {
            fn encode_using(&self, encoder: &mut Encoder) -> Result<(), EncodeError> {
                #encoder_calls
            }
        }
    })
}

pub fn derive_decode(stream: TokenStream) -> Result<TokenStream, String> {
    unimplemented!()
}
