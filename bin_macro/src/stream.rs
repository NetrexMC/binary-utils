use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, DeriveInput, Error, Fields, Result, Type, token::SelfType};

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
                         fn write(&self, &mut source: &mut Vec<u8>) {
                              #writes
                         }

                         fn read(source: &[u8], position: &mut usize) -> Self {
                              Self {
                                   #reads
                              }
                         }
                    }
               })
          },
          Data::Enum(v) => Err(syn::Error::new(name.span(), "BinaryStream does not support Enums. Use Structs instead.")),
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
          Fields::Unnamed(v) => {
               panic!("Can not parse un-named fields at this current point in time.")
          },
          Fields::Unit => {
               panic!("Can not use uninitalized data values.")
          }
     }
     (writers, readers)
}

pub fn impl_streamable_lazy(name: &Ident, ty: &Type) -> (TokenStream, TokenStream) {
     (quote!{ self.#name.write(&mut source); }, quote!(#name: #ty::read(&mut source, &mut position)))
}

