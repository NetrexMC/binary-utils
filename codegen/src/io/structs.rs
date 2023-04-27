use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use regex::Regex;
use syn::{DataStruct, Fields};

use crate::io::util::attrs::IoAttr;

use super::{util::attrs::resolve_generic_type, AstContext};
lazy_static! {
    static ref REG: regex::Regex = Regex::new(r"((?:self\.)([\u0041-\u323AF_0-9]*))").unwrap();
}

/// Derive structs will automatically implement the `BinaryReader` and `BinaryWriter` traits for the struct.
///
/// In the most generic example, we will parse a named struct:
/// ```ignore no_run
/// #[derive(BinaryIo)]
/// struct Test {
///    a: u8,
///    b: u16
/// }
/// ```
/// Where `Test` is the struct name, `a` and `b` are the field names, and `u8` and `u16` are the field types.
/// These fields will be parsed in order, and written in order.
///
/// The macro will also support unnamed structs:
/// ```ignore no_run
/// #[derive(BinaryIo)]
/// struct Test(u8, u16);
/// ```
/// Where `u8` and `u16` are the field types, and encoded in order.
/// Unfortunately the macro will not allow you to parse attributes on a struct with unnamed fields.
/// This is a limitation of the proc-macro system, and really shouldn't be abused.
pub(crate) fn derive_struct(
    ast_ctx: AstContext,
    data: DataStruct,
    error_stream: &mut TokenStream2,
) -> TokenStream {
    let struct_name = ast_ctx.0;
    let mut writer = TokenStream2::new();
    let mut reader = TokenStream2::new();
    // let constructing_idents: Vec<syn::Ident> = Vec::new();

    match data.fields {
        Fields::Named(ref fields) => {
            let field_names = fields
                .named
                .iter()
                .filter_map(|field| match field.ident {
                    Some(ref ident) => Some(ident),
                    None => {
                        error_stream.append_all(
                            syn::Error::new_spanned(
                                field,
                                "Cannot have unnamed fields in a struct!",
                            )
                            .to_compile_error(),
                        );
                        None
                    }
                })
                .collect::<Vec<&syn::Ident>>();

            for field in fields.named.iter() {
                let attributes = field
                    .attrs
                    .iter()
                    .filter_map(|att| {
                        match super::util::attrs::parse_attribute(&att, error_stream) {
                            Ok(attr) => Some(attr),
                            Err(_) => None,
                        }
                    })
                    .collect::<Vec<super::util::attrs::IoAttr>>();

                if attributes.len() > 1 {
                    error_stream.append_all(
                        syn::Error::new_spanned(
                            field,
                            "Cannot have more than one binary_utils Attribute on a field!",
                        )
                        .to_compile_error(),
                    );
                    return quote!().into();
                }

                // here we need to parse the field type
                let field_type = &field.ty;
                let field_name = &field.ident;

                if let Some(attr) = attributes.first() {
                    // we have an attribute, so we need to do some stuff with it before conditionally parsing.
                    match attr {
                        IoAttr::Require(id) => {
                            let inner_type: Option<syn::Type> =
                                resolve_generic_type(field_type, "Option", error_stream);

                            if inner_type.is_none() {
                                error_stream.append_all(syn::Error::new_spanned(
                                    field,
                                    "Cannot have a field with a binary_utils::Require attribute that is not an Option!"
                                ).to_compile_error());
                                return quote!().into();
                            }

                            let forced_type = inner_type.unwrap();

                            writer.append_all(quote!(
                                if self.#id.is_some() {
                                    _binary_writew.write(&mut (self.#field_name.unwrap()).write_to_bytes()?.as_slice())?;
                                } else {
                                    // return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, "Cannot write a field that is required but not present!"));
                                }
                            ));
                            reader.append_all(quote!(
                                if #id.is_none() {
                                    // return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, "Cannot read a field that is required but not present!"));
                                }
                                let #field_name = <#forced_type>::read(_binary_readerr).ok();
                            ));
                        }
                        IoAttr::Satisfy(expr) => {
                            let inner_type: Option<syn::Type> =
                                resolve_generic_type(field_type, "Option", error_stream);

                            if inner_type.is_none() {
                                error_stream.append_all(syn::Error::new_spanned(
                                    field,
                                    "Cannot have a field with a binary_utils::Satisfy attribute that is not an Option!"
                                ).to_compile_error());
                                return quote!().into();
                            }

                            // this is a conditional field! it requires the expression to be true when reading or writing.
                            let expr_tokens = expr.to_token_stream().to_string();
                            let p_wexp = expr_tokens.as_str();

                            let (write_capture, read_capture) = (
                                &REG.replace_all(p_wexp.clone(), r"self.$2"),
                                &REG.replace_all(p_wexp.clone(), r"$2"),
                            );
                            let (write_expr, read_expr) = (
                                syn::parse_str::<syn::Expr>(write_capture.as_ref()).unwrap(),
                                syn::parse_str::<syn::Expr>(read_capture.as_ref()).unwrap(),
                            );

                            writer.append_all(quote!(
                                if #write_expr {
                                    if let Some(v) = &self.#field_name {
                                        _binary_writew.write(&mut v.write_to_bytes()?.as_slice())?;
                                    } else {
                                        return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, format!("Condition for field {} was satisfied, but the field was not present!", stringify!(#field_name))));
                                    }
                                }
                            ));
                            reader.append_all(quote!(
                                // println!("{}: {}", stringify!(#field_name), stringify!(#read_expr));
                                let #field_name = match #read_expr {
                                    true => Some(<#inner_type>::read(_binary_readerr)?),
                                    false => None,
                                };
                            ));
                        }
                        IoAttr::Skip => {
                            // we skip this
                            writer.append_all(quote!(
                                // we skip this field
                            ));
                            reader.append_all(quote!(
                                // we skip this field
                                let #field_name: #field_type = Default::default();
                            ));
                            continue;
                        }
                    }
                } else {
                    // we don't have an attribute, so we just parse the field as normal interface type.
                    writer.append_all(quote!(
                        _binary_writew.write(&mut self.#field_name.write_to_bytes()?.as_slice())?;
                    ));
                    reader.append_all(quote!(
                        let #field_name = <#field_type>::read(_binary_readerr)?;
                    ));
                }
            }
            quote! {
                impl ::binary_utils::interfaces::Writer for #struct_name {
                    fn write(&self, _binary_writew: &mut ::binary_utils::io::ByteWriter) -> Result<(), ::std::io::Error> {
                        #writer
                        Ok(())
                    }
                }
                impl ::binary_utils::interfaces::Reader<#struct_name> for #struct_name {
                    fn read(_binary_readerr: &mut ::binary_utils::io::ByteReader) -> Result<#struct_name, ::std::io::Error> {
                        // println!("impl Reader for {} called!\n-> {}", stringify!(#struct_name), stringify!(#reader));
                        #reader
                        Ok(Self {
                            #(#field_names),*
                        })
                    }
                }
            }.into()
        }
        Fields::Unnamed(ref _fields) => {
            todo!("Implement named structs")
        }
        Fields::Unit => {
            error_stream.append_all(syn::Error::new_spanned(
                ast_ctx.0,
                "Unit structs are not supported by binary_utils because they have no fields to parse or write.\nThis may change in the future, but for now, please use a tuple struct instead."
            ).to_compile_error());
            return quote!().into();
        }
    }
}
