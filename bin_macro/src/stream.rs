use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Error, Fields, Result, Type};

pub fn stream_parse(input: DeriveInput) -> Result<TokenStream> {
    let name = &input.ident;
    let attrs = input.attrs;
    match input.data {
        Data::Struct(v) => {
            // iterate through struct fields
            let (w, r) = impl_named_fields(v.fields);
            let writes = quote!(#(#w)*);
            let reads = quote!(#(#r),*);
            // get the visibility etc on each field
            // return a quote for block impl
            Ok(quote! {
                 #[automatically_derived]
                 impl Streamable for #name {
                      fn parse(&self) -> Result<Vec<u8>, ::binary_utils::error::BinaryError> {
                           use ::std::io::Write;
                           use binary_utils::varint::{VarInt, VarIntWriter};
                           use binary_utils::u24::{u24, u24Writer};
                           let mut writer = Vec::new();
                           #writes
                           Ok(writer)
                      }

                      fn compose(source: &[u8], position: &mut usize) -> Result<Self, ::binary_utils::error::BinaryError> {
                           use ::std::io::Read;
                           use binary_utils::varint::{VarInt, VarIntReader};
                           use binary_utils::u24::{u24, u24Reader};

                           Ok(Self {
                                #reads
                           })
                      }
                 }
            })
        }
        Data::Enum(data) => {
            let representation = find_one_attr("repr", attrs)
                .expect("Enums must have a #[repr] attribute");
            let enum_ty = representation.parse_args::<Ident>()
                .expect("Enums can only have types as attributes");

            if !enum_ty.to_string().starts_with(|v| v == 'u' || v == 'i' || v == 'f') {
                return Err(Error::new_spanned(representation, "Representation must be a primitive number"));
            }

            let (mut writers, mut readers) = (Vec::<TokenStream>::new(), Vec::<TokenStream>::new());

            if !data.variants.iter().all(|v| match v.fields.clone() { Fields::Unit => true, Fields::Unnamed(_) => true, _ => false}) {
                return Err(Error::new_spanned(data.variants, "Enum Fields must be Uninitialized or Named"));
            }

            for variant in &data.variants {
                // for each field...
                let da = variant.discriminant.as_ref().expect("All Fields must have a explicit assignment.").clone();
                let discrim = da.1;
                let var_name = variant.ident.clone();
                match &variant.fields {
                    Fields::Unit => {
                        // writers
                        writers.push(quote!(Self::#var_name => Ok((#discrim as #enum_ty).parse()?),));
                        // readers
                        readers.push(quote!(#discrim => Ok(Self::#var_name),));
                    },
                    Fields::Unnamed(_fields) => {
                        return Err(Error::new_spanned(variant, "Variant fields are not explicitly supported yet."));
                        // for field in fields.unnamed.iter() {
                        //     dbg!("I am here 2\n\n\\nn\n\n");
                        // }
                    },
                    _ => return Err(Error::new_spanned(variant.clone(), "Variant invalid"))
                }
            }

            Ok(quote!{
                #[automatically_derived]
                impl Streamable for #name {
                    fn parse(&self) -> Result<Vec<u8>, ::binary_utils::error::BinaryError> {
                        match self {
                            #(#writers)*
                        }
                    }

                    fn compose(source: &[u8], offset: &mut usize) -> Result<Self, ::binary_utils::error::BinaryError> {
                        // get the repr type and read it
                        let v = <#enum_ty>::compose(source, offset)?;

                        match v {
                            #(#readers)*
                            _ => panic!("Will not fit in enum!")
                        }
                    }
                }
            })
        }
        Data::Union(_) => Err(syn::Error::new(
            name.span(),
            "BinaryStream does not support Type Unions. Use Enums instead.",
        )),
    }
}

pub fn impl_named_fields(fields: Fields) -> (Vec<TokenStream>, Vec<TokenStream>) {
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
        }
        Fields::Unnamed(_v) => {
            panic!("Can not parse un-named fields at this current point in time.")
        }
        Fields::Unit => {
            panic!("Can not use uninitalized data values.")
        }
    }
    (writers, readers)
}

// pub fn impl_unnamed_fields(_fields: FieldsUnnamed) -> (TokenStream, TokenStream) {

//     todo!()
// }

pub fn impl_streamable_lazy(name: &Ident, ty: &Type) -> (TokenStream, TokenStream) {
    (
        quote! { writer.write(&self.#name.parse()?[..])?; },
        quote!(#name: <#ty>::compose(&source, position)?),
    )
}

fn find_one_attr(name: &str, attrs: Vec<Attribute>) -> Option<Attribute> {
    let mut iter = attrs.iter().filter(|a| a.path.is_ident(name));
    match (iter.next(), iter.next()) {
        (Some(v), None) => Some(v.clone()),
        _ => None
    }
}