use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, Result, Type};

pub fn stream_parse(input: DeriveInput) -> Result<TokenStream> {
     let name = &input.ident;
     match input.data {
          Data::Struct(v) => {
               // iterate through struct fields
               let (w, r) = impl_struct_fields(v.fields);
               let writes = quote!(#(#w)*);
               let reads = quote!(#(#r),*);
               // get the visibility etc on each field
               // return a quote for block impl
               dbg!(&writes);
               Ok(quote! {
                    #[automatically_derived]
                    impl Streamable for #name {
                         fn parse(&self) -> Vec<u8> {
                              use ::std::io::Write;
                              use binary_utils::varint::{VarInt, VarIntWriter};
                              use binary_utils::u24::{u24, u24Writer};
                              let mut writer = Vec::new();
                              #writes
                              writer
                         }

                         fn compose(source: &[u8], position: &mut usize) -> Self {
                              use ::std::io::Read;
                              use binary_utils::varint::{VarInt, VarIntReader};
                              use binary_utils::u24::{u24, u24Reader};

                              Self {
                                   #reads
                              }
                         }
                    }
               })
          },
          Data::Enum(data) => {
               let mut write_collective = Vec::<TokenStream>::new();
               let mut read_collective = Vec::<TokenStream>::new();
               for var in data.variants.iter() {
                    let (w, r) = impl_named_fields(&var.ident, var.fields.clone());
                    write_collective.extend(w);
                    read_collective.extend(r);
               }
               let writes = quote!(#(#write_collective)*);
               let reads = quote!(#(#read_collective),*);
               // get the visibility etc on each field
               // return a quote for block impl
               Ok(quote! {
                    #[automatically_derived]
                    impl Streamable for #name {
                         fn parse(&self) -> Vec<u8> {
                              use ::std::io::Write;
                              use binary_utils::varint::{VarInt, VarIntWriter};
                              use binary_utils::u24::{u24, u24Writer};
                              let mut writer = Vec::new();
                              #writes
                              writer
                         }

                         fn compose(source: &[u8], position: &mut usize) -> Self {
                              use ::std::io::Read;
                              use binary_utils::varint::{VarInt, VarIntReader};
                              use binary_utils::u24::{u24, u24Reader};

                              Self {
                                   #reads
                              }
                         }
                    }
               })
          },
          Data::Union(_) => Err(syn::Error::new(name.span(), "BinaryStream does not support Type Unions. Use Enums instead."))
     }
}

pub fn impl_struct_fields(fields: Fields) -> (Vec<TokenStream>, Vec<TokenStream>) {
     let mut writers = Vec::<TokenStream>::new();
     let mut readers = Vec::<TokenStream>::new();
     match fields {
          Fields::Named(v) => {
               for field in &v.named {
                    let field_id = field.ident.as_ref().unwrap();
                    let (writer, reader) = impl_streamable_lazy(field_id, &field.ty);
                    writers.push(writer);
                    readers.push(reader);
               }
          },
          Fields::Unnamed(_v) => {
               panic!("Can not parse un-named fields at this current point in time.")
          },
          Fields::Unit => {
               panic!("Can not use uninitalized data values.")
          }
     }
     (writers, readers)
}

pub fn impl_named_fields(name: &Ident, fields: Fields) -> (Vec<TokenStream>, Vec<TokenStream>) {
     let mut writers = Vec::<TokenStream>::new();
     let mut readers = Vec::<TokenStream>::new();
     match fields {
          Fields::Named(v) => {
               for field in &v.named {
                    let field_id = field.ident.as_ref().unwrap();
                    let (writer, reader) = impl_streamable_lazy_named(name, field_id, &field.ty);
                    writers.push(writer);
                    readers.push(reader);
               }
          },
          Fields::Unnamed(_v) => {
               panic!("Can not parse un-named fields at this current point in time.")
          },
          Fields::Unit => {
               panic!("Can not use uninitalized data values.")
          }
     }
     (writers, readers)
}


pub fn impl_streamable_lazy(name: &Ident, ty: &Type) -> (TokenStream, TokenStream) {
     (quote!{ writer.write(&self.#name.parse()[..]); }, quote!(#name: #ty::compose(&source, position)))
}

pub fn impl_streamable_lazy_named(name: &Ident, named: &Ident, ty: &Type) -> (TokenStream, TokenStream) {
     (quote!{ writer.write(&self.#name.named.parse()[..]); }, quote!(#name: #ty::compose(&source, position)))
}
