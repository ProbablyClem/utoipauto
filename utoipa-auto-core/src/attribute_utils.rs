use quote::ToTokens;
use syn::Attribute;

/// Build the new openapi macro attribute with the newly discovered paths
pub fn build_new_openapi_attributes(src_uto_macro: String, uto_paths: &String) -> Attribute {
    let new_paths = format!("paths({}", uto_paths);

    let src_uto_macro = if !src_uto_macro.contains("paths(") {
        format!("{}), {}", new_paths, src_uto_macro)
    } else {
        src_uto_macro.replace("paths(", new_paths.as_str())
    };

    // let new_dto_schema = format!("schema({}", dto_paths);

    let stream: proc_macro2::TokenStream = src_uto_macro.parse().unwrap();

    syn::parse_quote! { #[openapi( #stream )] }
}

pub fn update_openapi_macro_attributes(macro_attibutes: &mut Vec<Attribute>, uto_paths: &String) {
    let mut is_ok = false;
    #[warn(clippy::needless_range_loop)]
    for i in 0..macro_attibutes.len() {
        if !macro_attibutes[i].path().is_ident("openapi") {
            continue;
        }
        is_ok = true;
        let mut src_uto_macro = macro_attibutes[i].to_token_stream().to_string();

        src_uto_macro = src_uto_macro.replace("#[openapi(", "");
        src_uto_macro = src_uto_macro.replace(")]", "");
        macro_attibutes[i] = build_new_openapi_attributes(src_uto_macro, uto_paths);
    }
    if !is_ok {
        panic!("No utoipa::openapi Macro found !");
    }
}

#[cfg(test)]
mod test {
    use quote::ToTokens;

    #[test]
    fn test_build_new_openapi_attributes() {
        assert_eq!(
            super::build_new_openapi_attributes("".to_string(), &"./src".to_string())
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "#[openapi(paths(./src),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_path_replace() {
        assert_eq!(
            super::build_new_openapi_attributes("paths(p1)".to_string(), &"./src,".to_string())
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "#[openapi(paths(./src,p1))]".to_string()
        );
    }
}
