use proc_macro::TokenStream;
use quote::quote;

mod io;
// mod legacy;

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
pub fn derive_stream(_input: TokenStream) -> TokenStream {
    return syn::Error::new_spanned(
        // parse_macro_input!(input as DeriveInput),
        quote!{},
        "This is a legacy proc-macro that is used to generate the BinaryStream\nDeprecated: use BinaryReader, and BinaryWriter instead."
    ).to_compile_error().into();
    // legacy::stream_parse(parse_macro_input!(input as DeriveInput))
    //     .unwrap()
    //     .into()
}

/// This proc-macro implements both the `Reader` and `Writer` traits from `binary_utils::interfaces`.
///
/// ## Structs
/// `BinaryIo` supports both Named, and Unnamed structs; however it does not support unit structs.
/// This macro will encode/decode the fields of the struct in the order they are defined, as long as they are not skipped;
/// however as an additional requirement, each field **MUST** implement** the `Reader` and `Writer` traits, if they do not, this macro will fail.
/// 
/// 
#[proc_macro_derive(BinaryIo, attributes(skip, require, if_present, satisfy))]
pub fn derive_binary_io(input: TokenStream) -> TokenStream {
    io::binary_encoder(input)
}
