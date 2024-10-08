#![forbid(unsafe_code)]
extern crate proc_macro;
use proc_macro::TokenStream;

#[allow(clippy::missing_panics_doc)]
#[proc_macro_derive(AnswerFn)]
pub fn derive_jtoo_pack(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
