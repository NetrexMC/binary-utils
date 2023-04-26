use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput, Data, Fields, DataStruct};

use super::AstContext;

pub(crate) fn derive_struct(ast_ctx: AstContext, data: DataStruct, error_stream: &mut TokenStream2) -> TokenStream {
    todo!()
}