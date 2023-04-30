use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use regex::Regex;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Fields};
use lazy_static::lazy_static;

use super::AstContext;

lazy_static! {
    static ref REG: regex::Regex = Regex::new(r"((?:self\.)([\u0041-\u323AF_0-9]*))").unwrap();
}

pub(crate) fn derive_enum(
    ast_ctx: AstContext,
    data: DataEnum,
    error_stream: &mut TokenStream2,
) -> TokenStream {
    todo!()
}