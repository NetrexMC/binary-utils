use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
mod stream;

#[proc_macro_derive(BinaryStream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
     stream::derive_stream(parse_macro_input!(input as DeriveInput)).into()
}