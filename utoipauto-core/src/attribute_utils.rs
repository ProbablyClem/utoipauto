use quote::ToTokens;
use syn::Attribute;

pub fn update_openapi_macro_attributes(
    macro_attibutes: &mut Vec<Attribute>,
    uto_paths: &String,
    uto_models: &String,
    uto_responses: &String,
) {
    let mut is_ok = false;
    for attr in macro_attibutes {
        if !attr.path().is_ident("openapi") {
            continue;
        }
        is_ok = true;
        let mut src_uto_macro = attr.to_token_stream().to_string();

        src_uto_macro = src_uto_macro.replace("#[openapi()]", "");
        src_uto_macro = src_uto_macro.replace("#[openapi(", "");
        src_uto_macro = src_uto_macro.replace(")]", "");
        *attr = build_new_openapi_attributes(src_uto_macro, uto_paths, uto_models, uto_responses);
    }
    if !is_ok {
        panic!("No utoipa::openapi Macro found !");
    }
}

/// Build the new openapi macro attribute with the newly discovered paths
pub fn build_new_openapi_attributes(
    src_uto_macro: String,
    uto_paths: &String,
    uto_models: &String,
    uto_responses: &String,
) -> Attribute {
    let paths = extract_paths(src_uto_macro.clone());
    let schemas = extract_schemas(src_uto_macro.clone());
    let responses = extract_responses(src_uto_macro.clone());
    let src_uto_macro = remove_paths(src_uto_macro);
    let src_uto_macro = remove_schemas(src_uto_macro);
    let src_uto_macro = remove_responses(src_uto_macro);
    let src_uto_macro = remove_components(src_uto_macro);

    let paths = format!("{}{}", uto_paths, paths);
    let schemas = format!("{}{}", uto_models, schemas);
    let responses = format!("{}{}", uto_responses, responses);
    let src_uto_macro = format!(
        "paths({}),components(schemas({}),responses({})),{}",
        paths, schemas, responses, src_uto_macro
    )
    .replace(",,", ",");

    let stream: proc_macro2::TokenStream = src_uto_macro.parse().unwrap();

    syn::parse_quote! { #[openapi( #stream )] }
}

fn remove_paths(src_uto_macro: String) -> String {
    if src_uto_macro.contains("paths(") {
        let paths = src_uto_macro.split("paths(").collect::<Vec<&str>>()[1];
        let paths = paths.split(')').collect::<Vec<&str>>()[0];
        src_uto_macro
            .replace(format!("paths({})", paths).as_str(), "")
            .replace(",,", ",")
    } else {
        src_uto_macro
    }
}

fn remove_schemas(src_uto_macro: String) -> String {
    if src_uto_macro.contains("schemas(") {
        let schemas = src_uto_macro.split("schemas(").collect::<Vec<&str>>()[1];
        let schemas = schemas.split(')').collect::<Vec<&str>>()[0];
        src_uto_macro
            .replace(format!("schemas({})", schemas).as_str(), "")
            .replace(",,", ",")
    } else {
        src_uto_macro
    }
}

fn remove_components(src_uto_macro: String) -> String {
    if src_uto_macro.contains("components(") {
        let components = src_uto_macro.split("components(").collect::<Vec<&str>>()[1];
        let components = components.split(')').collect::<Vec<&str>>()[0];
        src_uto_macro
            .replace(format!("components({})", components).as_str(), "")
            .replace(",,", ",")
    } else {
        src_uto_macro
    }
}

fn remove_responses(src_uto_macro: String) -> String {
    if src_uto_macro.contains("responses(") {
        let responses = src_uto_macro.split("responses(").collect::<Vec<&str>>()[1];
        let responses = responses.split(')').collect::<Vec<&str>>()[0];
        src_uto_macro
            .replace(format!("responses({})", responses).as_str(), "")
            .replace(",,", ",")
    } else {
        src_uto_macro
    }
}

fn extract_paths(src_uto_macro: String) -> String {
    if src_uto_macro.contains("paths(") {
        let paths = src_uto_macro.split("paths(").collect::<Vec<&str>>()[1];
        let paths = paths.split(')').collect::<Vec<&str>>()[0];
        paths.to_string()
    } else {
        "".to_string()
    }
}

fn extract_schemas(src_uto_macro: String) -> String {
    if src_uto_macro.contains("schemas(") {
        let schemas = src_uto_macro.split("schemas(").collect::<Vec<&str>>()[1];
        let schemas = schemas.split(')').collect::<Vec<&str>>()[0];
        schemas.to_string()
    } else {
        "".to_string()
    }
}

fn extract_responses(src_uto_macro: String) -> String {
    if src_uto_macro.contains("responses(") {
        let responses = src_uto_macro.split("responses(").collect::<Vec<&str>>()[1];
        let responses = responses.split(')').collect::<Vec<&str>>()[0];
        responses.to_string()
    } else {
        "".to_string()
    }
}
#[cfg(test)]
mod test {
    use quote::ToTokens;

    #[test]
    fn test_remove_paths() {
        assert_eq!(
            super::remove_paths("description(test),paths(p1),info(test)".to_string()),
            "description(test),info(test)".to_string()
        );
    }

    #[test]
    fn test_extract_paths() {
        assert_eq!(super::extract_paths("paths(p1)".to_string()), "p1".to_string());
    }

    #[test]
    fn test_extract_paths_empty() {
        assert_eq!(super::extract_paths("".to_string()), "".to_string());
    }

    #[test]
    fn test_build_new_openapi_attributes() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "".to_string(),
                &"./src".to_string(),
                &"".to_string(),
                &"".to_string(),
            )
                .to_token_stream()
                .to_string()
                .replace(' ', ""),
            "#[openapi(paths(./src),components(schemas(),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_path_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1)".to_string(),
                &"./src,".to_string(),
                &"".to_string(),
                &"".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1)".to_string(),
                &"./src,".to_string(),
                &"model".to_string(),
                &"".to_string()
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(model),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_schemas_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1), components(schemas(m1))".to_string(),
                &"./src,".to_string(),
                &"model,".to_string(),
                &"".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(model,m1),responses()),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1), components(responses(r1))".to_string(),
                &"./src,".to_string(),
                &"".to_string(),
                &"response,".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_schemas_replace() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1), components(responses(r1), schemas(m1))".to_string(),
                &"./src,".to_string(),
                &"model,".to_string(),
                &"response,".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(model,m1),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_responses_schemas() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1), components(responses(r1), schemas(m1))".to_string(),
                &"./src,".to_string(),
                &"".to_string(),
                &"response,".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(m1),responses(response,r1)),)]".to_string()
        );
    }

    #[test]
    fn test_build_new_openapi_attributes_components_schemas_reponses() {
        assert_eq!(
            super::build_new_openapi_attributes(
                "paths(p1), components(schemas(m1), responses(r1))".to_string(),
                &"./src,".to_string(),
                &"model,".to_string(),
                &"".to_string(),
            )
            .to_token_stream()
            .to_string()
            .replace(' ', ""),
            "#[openapi(paths(./src,p1),components(schemas(model,m1),responses(r1)),)]".to_string()
        );
    }
}
