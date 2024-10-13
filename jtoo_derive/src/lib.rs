//! This crate defines the derive macros for the [`jtoo`](https://crates.io/crates/jtoo)
//! `Encode` and `Decode` traits.
#![forbid(unsafe_code)]
extern crate proc_macro;
use proc_macro::TokenStream;

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Encode)]
pub fn jtoo_derive_encode(input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 =
        jtoo_derive_impl::derive_encode(input2).unwrap_or_else(syn::Error::into_compile_error);
    proc_macro::TokenStream::from(output2)
}

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(Decode)]
pub fn jtoo_derive_decode(input: TokenStream) -> TokenStream {
    let input2 = proc_macro2::TokenStream::from(input);
    let output2 = jtoo_derive_impl::derive_decode(input2).unwrap();
    proc_macro::TokenStream::from(output2)
}
