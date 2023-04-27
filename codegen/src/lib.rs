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
/// It is important to note that not all attributes can be used on all types, and some attributes are exclusive to certain variants.
///
/// ## Structs
/// `BinaryIo` supports both Named, and Unnamed structs. However, this derive macro does not support unit structs.
/// This macro will encode/decode the fields of the struct in the order they are defined, as long as they are not skipped;
/// however as an additional requirement, each field **MUST** implement** the `Reader` and `Writer` traits, if they do not, this macro will fail.
///
/// **Example:**
/// The following example will provide both a `Reader` and `Writer` implementation for the struct `ABC`, where each field is encoded as it's respective
/// type to the `Bytewriter`/`Bytereader`.
/// ```ignore
/// use binary_utils::interfaces::{Reader, Writer};
/// use binary_utils::BinaryIo;
///
/// #[derive(BinaryIo, Debug)]
/// struct ABC {
///    a: u8,
///    b: Option<u8>,
///    c: u8,
/// }
///```
///
/// Sometimes it can be more optimal to use Unnamed fields, if you do not care about the field names, and only want to encode/decode the fields in the order they are defined.
/// The behavior of this macro is the same as the previous example, except the fields are unnamed.
/// ```ignore
/// use binary_utils::interfaces::{Reader, Writer};
/// use binary_utils::BinaryIo;
///
/// #[derive(BinaryIo, Debug)]
/// struct ABC(u8, Option<u8>, u8);
/// ```
/// ---
///
/// ## Enums
/// Enums description has not been written as of yet...
///
/// ---
///
/// ## Attributes
/// Structs and enums have a few exclusive attributes that can be used to control the encoding/decoding of the struct. <br />
/// These attributes control and modify the behavior of the `BinaryIo` macro.
/// <br /><br />
///
/// ### Skip
/// The `#[skip]` attribute does as the name implies, and can be used to skip a field when encoding/decoding. <br />
///
/// **Syntax:**
/// ```rust
/// #[skip]
/// ```
///
/// **Compatibility:**
/// - ✅ Named Structs
/// - ✅ Unnamed Structs
/// - ✅ Enums
///
/// **Example:**
/// ```ignore
/// use binary_utils::interfaces::{Reader, Writer};
/// use binary_utils::BinaryIo;
///
/// #[derive(BinaryIo, Debug)]
/// struct ABC {
///     a: u8,
///     #[skip]
///     b: Option<u8>,
///     c: u8
/// }
/// ```
///
/// ### Require
/// This attribute explicitly requires a field to be present when either encoding, or decoding; and will fail if the field is not present. <br />
/// This can be useful if you want to ensure that an optional field is present when encoding, or decoding it.
///
/// **Syntax:**
/// ```rust
/// #[require(FIELD)]
/// ```
///
/// **Compatibility:**
/// - ✅ Named Structs
/// - ❌ Unnamed Structs
/// - ✅ Enums
///
/// **Example:**
/// In the following example, `b` is explicitly required to be present when encoding, or decoding `ABC`, and it's value is not allowed to be `None`.
/// ```ignore
/// use binary_utils::interfaces::{Reader, Writer};
/// use binary_utils::BinaryIo;
///
/// #[derive(BinaryIo, Debug)]
/// struct ABC {
///     a: u8,
///     b: Option<u8>,
///     #[require(b)]
///     c: Option<u8>
/// }
/// ```
///
/// ### If Present
/// This attribute functions identically to `#[require]`, however it does not fail if the field is not present.
///
/// ### Satisfy
/// This attribute will fail if the expression provided does not evaluate to `true`. <br />
/// This attribute can be used to ensure that a field is only encoded/decoded if a certain condition is met.
/// This can be useful if you're sending something like `Authorization` or `Authentication` packets, and you want to ensure that the client is authenticated before
/// sending the packet.
///
/// **Syntax:**
///
/// ```rust
/// #[satisfy(EXPR)]
/// ```
/// **Compatibility:**
/// - ✅ Named Structs
/// - ❌ Unnamed Structs
/// - ✅ Enums
///
/// **Example:**
/// ```ignore
/// #[derive(BinaryIo, Debug)]
/// struct ABC {
///     a: u8,
///     #[satisfy(self.a == 10)]
///     b: Option<u8>,
///     c: u8,
/// }
/// ```
/// ---
///
#[proc_macro_derive(BinaryIo, attributes(skip, require, if_present, satisfy))]
pub fn derive_binary_io(input: TokenStream) -> TokenStream {
    io::binary_encoder(input)
}
