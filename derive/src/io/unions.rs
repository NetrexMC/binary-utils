use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::DataUnion;

use super::AstContext;

pub(crate) fn derive_union(ast_ctx: AstContext, _: DataUnion, _: &mut TokenStream2) -> TokenStream {
    syn::Error::new_spanned(
        ast_ctx.0,
        "Unions are not supported by binary_util, there is currently no way to implement the BinaryReader and BinaryWriter traits for unions."
    ).to_compile_error().into()
}
