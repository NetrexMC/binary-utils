use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod legacy;

/// **DEPRECATED**.
/// This is a legacy proc-macro that is used to generate a BufferStream.
/// It provides an easy way to implement the `Streamable` trait.
///
/// ## Deprecated
/// Deprecated since `0.3.0` in favor of `BinaryReader` and `BinaryWriter`.
///
/// Example:
/// ```ignore
/// use binary_utils::BinaryStream;
///
/// #[derive(BinaryStream)]
/// struct Test {
///    a: u8,
///    b: u16
/// }
///
/// fn main() {
///   let test = Test { a: 0, b: 0 };
///   test.parse().unwrap();
/// }
/// ```
#[proc_macro_derive(BinaryStream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    return syn::Error::new_spanned(
        parse_macro_input!(input as DeriveInput),
        "This is a legacy proc-macro that is used to generate the BinaryStream\nDeprecated: use BinaryReader, and BinaryWriter instead."
    ).to_compile_error().into();
    // legacy::stream_parse(parse_macro_input!(input as DeriveInput))
    //     .unwrap()
    //     .into()
}