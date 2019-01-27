#![recursion_limit = "128"]

extern crate proc_macro;

mod imp;

use crate::proc_macro::TokenStream;
use syn;

#[proc_macro_derive(EnumBits)]
pub fn enum_bits_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    imp::impl_enum_bits_macro(&ast).into()
}
