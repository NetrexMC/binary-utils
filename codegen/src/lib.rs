use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod io;
mod legacy;

/// **DEPRECATED**.
/// This is a legacy proc-macro that is used to generate a BufferStream.
/// It provides an easy way to implement the `Streamable` trait.
/// > ⚠️ This proc-macro has been deprecated since `0.3.0` in favor of `binary_util::interfaces::Reader` and `binary_util::interfaces::Writer` and will be removed in `0.4.0`.
///
/// This proc-macro automatically implements the `Streamable` trait for the struct or enum it is applied to.
///
/// Example:
/// ```ignore
/// use binary_util::BinaryStream;
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
///
/// Please note that this proc-macro does not support unit structs or named enum variants, meaning a code sample like the following will not work:
/// ```warn
/// use binary_util::BinaryStream;
///
/// // Error: Unit structs are not supported.
/// #[derive(BinaryStream)]
/// struct Test;
///
/// // Error: Invalid variant.
/// #[derive(BinaryStream)]
/// enum TestEnum {
///     B(Test)
/// }
/// ```
#[proc_macro_derive(BinaryStream)]
pub fn derive_stream(input: TokenStream) -> TokenStream {
    // return syn::Error::new_spanned(
    //     // parse_macro_input!(input as DeriveInput),
    //     quote!{},
    //     "This is a legacy proc-macro that is used to generate the BinaryStream\nDeprecated: use BinaryReader, and BinaryWriter instead."
    // ).to_compile_error().into();
    legacy::stream_parse(parse_macro_input!(input as DeriveInput))
        .unwrap()
        .into()
}

/// This proc-macro implements both the `Reader` and `Writer` traits from `binary_util::interfaces`.
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
/// use binary_util::interfaces::{Reader, Writer};
/// use binary_util::BinaryIo;
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
/// use binary_util::interfaces::{Reader, Writer};
/// use binary_util::BinaryIo;
///
/// #[derive(BinaryIo, Debug)]
/// struct ABC(u8, Option<u8>, u8);
/// ```
/// ---
///
/// ## Enums
/// Enums function a bit differently than structs, and have a few more exclusive attributes that allow you to adjust the behavior of the macro.
/// Identically to structs, this macro will encode/decode the fields of the enum in the order they are defined, as long as they are not skipped.
/// > **Note:** Enums require the `#[repr]` attribute to be used, and the `#[repr]` attribute must be a primitive type.
///
/// ### Unit Variants
/// Unit variants are the simplest variant, of an enum and require the `#[repr(usize)]` attribute to be used. <br />
///
/// **Example:**
/// The following example will encode the `ProtcolEnum` enum as a `u8`, where each variant is encoded, by default, starting from 0.
///
/// ```ignore
/// use binary_util::BinaryIo;
/// use binary_util::{Reader, Writer};
///
/// #[derive(BinaryIo, Debug)]
/// #[repr(u8)]
/// pub enum ProtocolEnum {
///     Basic,
///     Advanced,
///     Complex
/// }
/// ```
///
/// ### Unnamed Variants (Tuple)
/// Unnamed variants allow you to encode the enum with a byte header specified by the discriminant. <br />
/// However, this variant is limited to the same functionality as a struct. The containing data of each field
/// within the variant must implement the `Reader` and `Writer` traits. Otherwise, this macro will fail with an error.
///
/// **Example:**
/// The following example makes use of Unnamed variants, in this case `A` to encode both `B` and `C` retrospectively.
/// Where `A::JustC` will be encoded as `0x02` with the binary data of struct `B`.
/// ```ignore
/// use binary_util::BinaryIo;
/// use binary_util::{Reader, Writer};
///
/// #[derive(BinaryIo, Debug)]
/// pub struct B {
///     foo: String,
///     bar: Vec<u8>
/// }
///
/// #[derive(BinaryIo, Debug)]
/// pub struct C {
///     foobar: u32,
/// }
///
/// #[derive(BinaryIo, Debug)]
/// #[repr(u8)]
/// pub enum A {
///     JustB(B) = 1,
///     JustC(C), // 2
///     Both(B, C) // 3
/// }
///
/// fn main() {
///     let a = A::JustC(C { foobar: 4 });
///     let buf = a.write_to_bytes().unwrap();
///
///     assert_eq!(buf, &[2, 4, 0, 0, 0]);
/// }
/// ```
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
/// use binary_util::interfaces::{Reader, Writer};
/// use binary_util::BinaryIo;
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
/// - ❌ Enums
///
/// **Example:**
/// In the following example, `b` is explicitly required to be present when encoding, or decoding `ABC`, and it's value is not allowed to be `None`.
/// ```ignore
/// use binary_util::interfaces::{Reader, Writer};
/// use binary_util::BinaryIo;
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
/// - ❌ Enums
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
