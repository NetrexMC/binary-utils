pub(crate) mod enums;
pub(crate) mod structs;
pub(crate) mod unions;
pub(crate) mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

pub(crate) type AstContext<'a> = (
    &'a syn::Ident,
    &'a Vec<syn::Attribute>,
    &'a syn::Generics,
    &'a syn::Visibility,
);

// BinaryEncoder is a derive macro that implements `::binary_utils::interfaces::Reader<T>` and `::binary_utils::interfaces::Writer<T>`
pub(crate) fn binary_encoder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ctx: AstContext = (&input.ident, &input.attrs, &input.generics, &input.vis);

    let mut err = proc_macro2::TokenStream::new();

    let stream = match input.data {
        Data::Struct(d) => structs::derive_struct(ctx, d, &mut err),
        Data::Enum(d) => enums::derive_enum(ctx, d, &mut err),
        Data::Union(d) => unions::derive_union(ctx, d, &mut err),
    };

    if err.is_empty() {
        stream.into()
    } else {
        err.into()
    }
}
