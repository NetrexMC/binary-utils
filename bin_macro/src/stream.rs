use proc_macro2::{Ident, Literal, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, Result};

pub fn stream_parse(input: DeriveInput) -> Result<TokenStream> {
     let name = &input.ident;
     match input.data {
          Data::Struct(v) => {

          },
          Data::Enum(v) => {

          },
          _ => panic!("BinaryStream does not support Type Unions. Use Enums instead.")
     }
     Ok(quote! {})
}

pub fn impl_struct_fields(fields: Fields) {
     match fields {
          Fields::Named(v) => {
               for field in &v.named {
                    let field_id = field.ident.as_ref();
                    let id = quote!(self.#field_id);
                    
               }
          },
          Fields::Unnamed(v) => {

          },
          Fields::Unit => {
               panic!("Can not use uninitalized data values.")
          }
     }
}