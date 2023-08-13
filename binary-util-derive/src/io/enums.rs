#![allow(dead_code)]
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use regex::Regex;
use syn::{DataEnum, Error, Fields};

use super::util::attrs::{parse_attribute, IoAttr};

use super::AstContext;

lazy_static! {
    // this regex checks whether a repr attribute a valid repr attribute
    static ref REG: regex::Regex = Regex::new(r"^(?:u|i)(?:size|8|16|32|32|64|128)$").unwrap();
}

/// A helper struct for parsing enum variants.
/// This struct is used internally, however, for those who wish to modify this struct documentation is provided.
///
/// This struct is created AFTER the attribute of the variant is parsed.
/// Parsing Order (pesudo-code):
/// ```md
/// for (variant in enum) {
///     discriminat = parse_discriminant(variant) || current_discriminant.next();
///     parse_attribute(variant);
///     parse_variant(variant, attributes, discriminant); // HERE
/// }
/// ```
struct ParsedEnumVariant {
    /// The name of the variant.
    pub name: syn::Ident,
    /// The contents to append when writing. this variant.
    /// This content will be appended within the expression of the match arm.
    /// IE `<HERE>` in the following example:
    /// ```ignore
    /// #[repr(u8)]
    /// enum MyEnum {
    ///    Unit, // 0
    ///    UintDiscrm = 2,
    ///    Variant(String, u8), // 3
    /// }
    ///
    /// impl Writer for MyEnum {
    ///     pub fn write(&self, writer: &mut ByteWriter) -> Result<(), std::io::Error> {
    ///         match self {
    ///             MyEnum::Unit => {
    ///                // <HERE>
    ///             },
    ///             MyEnum::UintDiscrm => {
    ///                // <HERE>
    ///             },
    ///             // etc...
    ///             MyEnum::Variant(arg1, arg2) => {
    ///                // You can explect each argument to be a `syn::Ident` with the prefix `arg` followed
    ///                // followed by the index of the argument in the variant.
    ///                // <HERE>
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub write_content: TokenStream2,
    /// The contents to append when reading this variant.
    /// Similar to `write_content`, this content will be appended within the expression of the match arm.
    /// IE `<HERE>` in the following example:
    /// ```ignore
    /// #[repr(u8)]
    /// enum MyEnum {
    ///     Unit, // 0
    ///     UintDiscrm = 2,
    ///     Variant(String, u8), // 3
    /// }
    /// impl Reader<MyEnum> for MyEnum {
    ///     fn read(&self, reader: &mut ByteReader) -> Result<MyEnum, std::io::Error> {
    ///         let discriminant = reader.read_usize()?;
    ///         match discriminant {
    ///             // here
    ///             0 => {}
    ///             // etc...
    ///         }
    ///     }
    /// }
    /// ```
    pub read_content: TokenStream2,
    /// The discriminant of this variant.
    /// This is automatically set by the parser.
    pub discriminant: syn::LitInt,
}

pub(crate) fn derive_enum(
    ast_ctx: AstContext,
    data: DataEnum,
    error_stream: &mut TokenStream2,
) -> TokenStream {
    // The name of our enum.
    let enum_name = ast_ctx.0;

    // get the repr attribute if it exists
    let repr = ast_ctx
        .1
        .iter()
        .filter(|attr| attr.path().is_ident("repr"))
        .next();

    // if there's no repr, we're using u8, otherwise we're using the repr specified.
    let repr_type = match repr {
        Some(repr) => {
            let repr = repr.parse_args::<syn::Ident>().unwrap();
            // todo validate the repr
            repr
        }
        None => {
            // we need to force the user to specify a repr attribute
            error_stream.append_all(
                Error::new_spanned(&enum_name, "Enum must have a #[repr] attribute.")
                    .to_compile_error(),
            );
            return TokenStream::new();
        }
    };

    if !REG.is_match(&repr_type.to_string()) {
        error_stream.append_all(
            Error::new_spanned(
                &repr_type,
                "#[repr] attribute must contain a valid C type, one of: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize"
            )
            .to_compile_error()
        );
        return TokenStream::new();
    }

    let mut curr_discrim: Option<i128> = None;

    let mut variants: Vec<ParsedEnumVariant> = Vec::new();

    for variant in data.variants.iter() {
        // parse the discriminant
        if let Some((_, expr)) = &variant.discriminant {
            // check whether the expression is a syn::LitInt
            match syn::parse::<syn::LitInt>(expr.to_token_stream().into()) {
                Ok(v) => {
                    if let Ok(discrim) = v.base10_parse::<i128>() {
                        curr_discrim = Some(discrim);
                    } else {
                        error_stream.append_all(
                            Error::new_spanned(
                                expr,
                                "Discriminant must be a primitive integer literal.",
                            )
                            .to_compile_error(),
                        );
                        return TokenStream::new();
                    }
                }
                Err(_) => {
                    error_stream.append_all(
                        Error::new_spanned(
                            expr,
                            "Discriminant must be a primitive integer literal.",
                        )
                        .to_compile_error(),
                    );
                    return TokenStream::new();
                }
            }
        } else {
            // no discriminant, so we use the next discriminant
            // todo parse descend vs ascend, currently the order doesnt matter.
            curr_discrim = match curr_discrim {
                Some(discrim) => Some(discrim + 1),
                None => Some(0),
            }
        }

        // parse the attributes
        let attributes = variant
            .attrs
            .iter()
            .filter_map(|att| match parse_attribute(&att, error_stream) {
                Ok(attr) => match attr {
                    IoAttr::Unknown => None,
                    IoAttr::Doc(_) => None,
                    _ => Some(attr),
                },
                Err(_) => None,
            })
            .collect::<Vec<super::util::attrs::IoAttr>>();

        if let Some(attr) = attributes.first() {
            match *attr {
                IoAttr::Skip => {}
                IoAttr::Satisfy(_) | IoAttr::IfPresent(_) | IoAttr::Require(_) => {
                    error_stream.append_all(
                        Error::new_spanned(
                            &variant,
                            "Attributes: #[satisfy], #[if_present], and #[require] are not valid on enum variants."
                        )
                        .to_compile_error()
                    );
                    return TokenStream::new();
                }
                _ => {}
            }
        }

        // todo support parsing of named fields
        // these are fields like the following
        // enum MyEnum {
        //   Test { a: u8, b: u8 }
        // }
        if let Fields::Named(_) = variant.fields {
            error_stream.append_all(
                Error::new_spanned(
                    &variant.fields,
                    "Enums can not have named fields in their variants. See https://github.com/NetrexMC/binary-utils/issues/15"
                )
                .to_compile_error()
            );
            return TokenStream::new();
        }

        // we need to parse this indo an ident _ and a type
        let di = format!("{}{}", curr_discrim.unwrap(), repr_type);
        let discrim = syn::LitInt::new(&di, proc_macro2::Span::call_site());

        // we need to iterate through each field and parse it.
        // keep in mind, in this context we're inside of the expr within the variant
        // ie:
        // enum MyEnum {
        //   Test(HERE)
        // }
        variants.push(parse_enum_variant(
            variant,
            &attributes,
            &discrim,
            error_stream,
        ));

        // this is hacky, but we need to check whether or not the error_stream has any errors.
        // if it does, we need to return an empty token stream.
        if !error_stream.is_empty() {
            return TokenStream::new();
        }
    }

    // get all write streams from variants
    let write_streams = variants
        .iter()
        .map(|variant| variant.write_content.clone())
        .collect::<Vec<TokenStream2>>();
    let read_streams = variants
        .iter()
        .map(|variant| variant.read_content.clone())
        .collect::<Vec<TokenStream2>>();

    quote! {
        impl ::binary_util::interfaces::Writer for #enum_name {
            fn write(&self, _binary_writew: &mut ::binary_util::io::ByteWriter) -> ::std::result::Result<(), ::std::io::Error> {
                match self {
                    #(#write_streams)*
                };

                Ok(())
            }
        }

        impl ::binary_util::interfaces::Reader<#enum_name> for #enum_name {
            fn read(_binary_readerr: &mut ::binary_util::io::ByteReader) -> ::std::result::Result<#enum_name, ::std::io::Error> {
                match <#repr_type>::read(_binary_readerr)? {
                    #(#read_streams)*
                    _ => Err(::std::io::Error::new(::std::io::ErrorKind::InvalidData, "Invalid enum discriminant."))
                }
            }
        }
    }.into()
}

fn parse_enum_variant(
    variant: &syn::Variant,
    _attributes: &Vec<super::util::attrs::IoAttr>,
    curr_discrim: &syn::LitInt,
    error_stream: &mut TokenStream2,
) -> ParsedEnumVariant {
    let mut read_content = TokenStream2::new();
    let mut write_content = TokenStream2::new();

    let variant_name = &variant.ident;

    match variant.fields {
        Fields::Unnamed(ref fields) => {
            // This is the stream within the match arm
            let mut read_inner = TokenStream2::new();
            let mut write_inner = TokenStream2::new();

            let mut args: Vec<syn::Ident> = Vec::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let inner_attrs = field
                    .attrs
                    .iter()
                    .filter_map(|att| match parse_attribute(&att, error_stream) {
                        Ok(attr) => match attr {
                            IoAttr::Unknown => None,
                            _ => Some(attr),
                        },
                        Err(_) => None,
                    })
                    .collect::<Vec<super::util::attrs::IoAttr>>();

                if inner_attrs.len() != 0 {
                    error_stream.append_all(
                        syn::Error::new_spanned(
                            field,
                            "Attributes are not valid on enum variant fields at this time.",
                        )
                        .to_compile_error(),
                    );
                    break;
                }

                let arg_type = &field.ty;
                let arg_name = format_ident!("arg{}", i);

                write_inner.append_all(quote! {
                    _binary_writew.write(&mut #arg_name.write_to_bytes()?.as_slice())?;
                });
                read_inner.append_all(quote! {
                    let #arg_name = <#arg_type>::read(_binary_readerr)?;
                });

                args.push(arg_name);
            }

            write_content.append_all(quote!(
                Self::#variant_name(#(#args),*) => {
                    _binary_writew.write(&mut #curr_discrim.write_to_bytes()?.as_slice())?;
                    #write_inner
                }
            ));
            read_content.append_all(quote!(
                #curr_discrim => {
                    #read_inner
                    Ok(Self::#variant_name(#(#args),*))
                }
            ));
        }
        Fields::Unit => {
            // Unit variants are easy, we just read/write the discriminant.
            read_content.append_all(quote! {
                #curr_discrim => Ok(Self::#variant_name),
            });
            write_content.append_all(quote! {
                Self::#variant_name => {
                    _binary_writew.write(&mut #curr_discrim.write_to_bytes()?.as_slice())?;
                },
            });
        }
        _ => {
            error_stream.append_all(
                Error::new_spanned(&variant.fields, "Something went really wrong..")
                    .to_compile_error(),
            );
        }
    }

    ParsedEnumVariant {
        name: variant.ident.clone(),
        read_content,
        write_content,
        discriminant: syn::parse::<syn::LitInt>(quote!(#curr_discrim).into()).unwrap(),
    }
}
