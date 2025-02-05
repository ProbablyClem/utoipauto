use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, Attribute, Meta, Token};

pub fn update_openapi_macro_attributes(
    macro_attibutes: &mut Vec<Attribute>,
    uto_paths: &TokenStream,
    uto_models: &TokenStream,
    uto_responses: &TokenStream,
) {
    let mut is_ok = false;
    for attr in macro_attibutes {
        if !attr.path().is_ident("openapi") {
            continue;
        }
        is_ok = true;
        match &attr.meta {
            // #[openapi]
            Meta::Path(_path) => {
                *attr = build_new_openapi_attributes(Punctuated::new(), uto_paths, uto_models, uto_responses);
            }
            // #[openapi()] or #[openapi(attribute(...))]
            Meta::List(meta_list) => {
                let nested = meta_list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .expect("Expected a list of attributes inside #[openapi(...)]!");
                *attr = build_new_openapi_attributes(nested, uto_paths, uto_models, uto_responses);
            }
            // This would be #[openapi = "foo"], which is not valid
            Meta::NameValue(_) => panic!("Expected #[openapi(...)], but found #[openapi = value]!"),
        }
    }
    if !is_ok {
        panic!("No utoipa::openapi Macro found !");
    }
}

/// Build the new openapi macro attribute with the newly discovered paths
pub fn build_new_openapi_attributes(
    nested_attributes: Punctuated<Meta, Token![,]>,
    uto_paths: &TokenStream,
    uto_models: &TokenStream,
    uto_responses: &TokenStream,
) -> Attribute {
    let paths = extract_paths(&nested_attributes);
    let schemas = extract_schemas(&nested_attributes);
    let responses = extract_responses(&nested_attributes);
    let remaining_nested_attributes = remove_paths_and_components(nested_attributes);

    let uto_paths = match uto_paths.is_empty() {
        true => TokenStream::new(),
        false => quote::quote!(#uto_paths,),
    };
    let uto_models = match uto_models.is_empty() {
        true => TokenStream::new(),
        false => quote::quote!(#uto_models,),
    };
    let uto_responses = match uto_responses.is_empty() {
        true => TokenStream::new(),
        false => quote::quote!(#uto_responses,),
    };
    let uto_macro = quote::quote!(
        paths(#uto_paths #paths),components(schemas(#uto_models #schemas),responses(#uto_responses #responses)),
        #remaining_nested_attributes
    );

    syn::parse_quote! { #[openapi( #uto_macro )] }
}

fn remove_paths_and_components(nested_attributes: Punctuated<Meta, Token![,]>) -> TokenStream {
    let mut remaining = Vec::with_capacity(nested_attributes.len());
    for meta in nested_attributes {
        match meta {
            Meta::List(list) if list.path.is_ident("paths") => (),
            Meta::List(list) if list.path.is_ident("components") => (),
            // These should be handled by removing `components`, this is just in case they occur outside of `components` for some reason.
            Meta::List(list) if list.path.is_ident("schemas") => (),
            Meta::List(list) if list.path.is_ident("responses") => (),
            _ => remaining.push(meta),
        }
    }
    quote::quote!( #(#remaining),* )
}

fn extract_paths(nested_attributes: &Punctuated<Meta, Token![,]>) -> TokenStream {
    for meta in nested_attributes {
        if let Meta::List(list) = meta {
            if list.path.is_ident("paths") {
                return list.tokens.clone();
            }
        }
    }

    TokenStream::new()
}

fn extract_schemas(nested_attributes: &Punctuated<Meta, Token![,]>) -> TokenStream {
    for meta in nested_attributes {
        if let Meta::List(list) = meta {
            if list.path.is_ident("components") {
                let nested = list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .expect("Expected a list of attributes inside components(...)!");
                for meta in nested {
                    if let Meta::List(list) = meta {
                        if list.path.is_ident("schemas") {
                            return list.tokens;
                        }
                    }
                }
            }
        }
    }
    TokenStream::new()
}

fn extract_responses(nested_attributes: &Punctuated<Meta, Token![,]>) -> TokenStream {
    for meta in nested_attributes {
        if let Meta::List(list) = meta {
            if list.path.is_ident("components") {
                let nested = list
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .expect("Expected a list of attributes inside components(...)!");
                for meta in nested {
                    if let Meta::List(list) = meta {
                        if list.path.is_ident("responses") {
                            return list.tokens;
                        }
                    }
                }
            }
        }
    }
    TokenStream::new()
}
#[cfg(test)]
mod test {
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::punctuated::Punctuated;

    #[test]
    fn test_extract_paths() {
        assert_eq!(
            super::extract_paths(&syn::parse_quote!(paths(p1))).to_string(),
            "p1".to_string()
        );
    }

    #[test]
    fn test_extract_paths_empty() {
        assert_eq!(super::extract_paths(&Punctuated::new()).to_string(), "".to_string());
    }

    #[test]
    fn test_build_new_openapi_attributes() {
        assert_eq!(
            super::build_new_openapi_attributes(
                Punctuated::new(),
                &quote::quote!(crate::api::test),
                &TokenStream::new(),
                &TokenStream::new(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,),components(schemas(),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_path_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1)),
                &quote::quote!(crate::api::test),
                &TokenStream::new(),
                &TokenStream::new(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1)),
                &quote::quote!(crate::api::test),
                &quote::quote!(model),
                &TokenStream::new(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(model,),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_schemas_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1), components(schemas(m1))),
                &quote::quote!(crate::api::test),
                &quote::quote!(model),
                &TokenStream::new(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(model,m1),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1), components(responses(r1))),
                &quote::quote!(crate::api::test),
                &TokenStream::new(),
                &quote::quote!(response),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_schemas_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1), components(responses(r1), schemas(m1))),
                &quote::quote!(crate::api::test),
                &quote::quote!(model),
                &quote::quote!(response),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(model,m1),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_schemas() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1), components(responses(r1), schemas(m1))),
                &quote::quote!(crate::api::test),
                &TokenStream::new(),
                &quote::quote!(response),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(m1),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_schemas_reponses() {
        assert_eq!(
            super::build_new_openapi_attributes(
                syn::parse_quote!(paths(p1), components(schemas(m1), responses(r1))),
                &quote::quote!(crate::api::test),
                &quote::quote!(model),
                &TokenStream::new(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(model,m1),responses(r1)),)]".to_string()
        );
    }

    #[test]
    fn test_update_openapi_attributes_empty() {
        let mut attrs = vec![syn::parse_quote!(#[openapi])];
        super::update_openapi_macro_attributes(
            &mut attrs,
            &quote::quote!(crate::api::test),
            &quote::quote!(model),
            &TokenStream::new(),
        );
        assert_eq!(
            attrs[0].to_token_stream().to_string().replace(' ', ""),
            "#[openapi(paths(crate::api::test,),components(schemas(model,),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_update_openapi_attributes_components_schemas_reponses() {
        let mut attrs = vec![syn::parse_quote!(#[openapi(paths(p1), components(schemas(m1), responses(r1)))])];
        super::update_openapi_macro_attributes(
            &mut attrs,
            &quote::quote!(crate::api::test),
            &quote::quote!(model),
            &TokenStream::new(),
        );
        assert_eq!(
            attrs[0].to_token_stream().to_string().replace(' ', ""),
            "#[openapi(paths(crate::api::test,p1),components(schemas(model,m1),responses(r1)),)]".to_string()
        );
    }
}
