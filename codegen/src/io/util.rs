pub(crate) mod attrs {
    use proc_macro2::TokenStream as TokenStream2;
    use quote::TokenStreamExt;

    use lazy_static::lazy_static;
    use regex::Regex;

    lazy_static! {
        static ref REG: regex::Regex = Regex::new(r"\$([\u0041-\u323AF|_]*)").unwrap();
    }

    #[derive(Clone)]
    pub enum IoAttr {
        Satisfy(syn::Expr),
        Require(syn::Ident),
        Skip,
    }

    /// Parses the attributes of a struct or enum.
    /// The attributes are returned in the order they were parsed in, you can return errors if you want to.
    /// Some attributes do not allow conflicting attributes, such as #[skip]
    pub fn parse_attribute<'a>(
        attr: &'a syn::Attribute,
        error_stream: &mut TokenStream2,
    ) -> Result<IoAttr, ()> {
        let path = attr.path();
        if path.is_ident("satisfy") {
            // Satisfy is an attribute that allows an expression to be specified
            // this is polyfilled later with `self.EXPRESSION`
            match attr.parse_args::<syn::Expr>() {
                Ok(expr) => {
                    return Ok(IoAttr::Satisfy(expr));
                }
                Err(e) => {
                    error_stream.append_all(
                        syn::Error::new_spanned(attr, format!("Satisfy attribute requires an Expression!\n Example: #[satisfy(self.field == 0)]\n Error: {}", e))
                            .to_compile_error(),
                    );
                }
            }
        } else if path.is_ident("require") {
            // Require is an attribute that allows an identifier to be specified
            // this is polyfilled later with `self.IDENTIFIER.is_some()`
            match attr.parse_args::<syn::Ident>() {
                Ok(ident) => {
                    return Ok(IoAttr::Require(ident));
                }
                Err(_) => {
                    error_stream.append_all(
                        syn::Error::new_spanned(attr, "Require attribute requires an Identifier! \n Example: #[require(self.field)]")
                            .to_compile_error(),
                    );
                }
            }
        } else if path.is_ident("skip") {
            // skip is a special attribute, it cannot be used with any other attribute
            // therefore we can just return early, however we need to validate that
            // there are no other attributes
            return Ok(IoAttr::Skip);
        } else {
            error_stream.append_all(
                syn::Error::new_spanned(
                    attr,
                    "Unknown attribute, did you mean 'satisfy', 'require', or 'skip'?",
                )
                .to_compile_error(),
            );
        }

        Err(())
    }

    /// Parses the attributes of a struct or enum.
    /// todo: this is a bit of a mess, and should be cleaned up.
    /// todo: There's probably a better way to resolve the type without having to do this.
    pub fn resolve_generic_type<'a>(
        ty: &'a syn::Type,
        ident: &str,
        error_stream: &mut TokenStream2,
    ) -> Option<syn::Type> {
        match *ty {
            syn::Type::Path(ref tp) => {
                if let Some(first) = tp.path.segments.first() {
                    if first.ident.to_string() == ident {
                        if let syn::PathArguments::AngleBracketed(args) = &first.arguments {
                            if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                Some(inner.clone())
                            } else {
                                error_stream.append_all(syn::Error::new_spanned(
                                    ty,
                                    "Option type must have a generic argument in order to be required!"
                                ).to_compile_error());
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
