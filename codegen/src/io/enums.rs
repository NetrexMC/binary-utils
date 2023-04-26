use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataEnum};

use super::AstContext;

pub(crate) fn derive_enum(ast_ctx: AstContext, data: DataEnum, error_stream: &mut TokenStream2) -> TokenStream {
    todo!()
}