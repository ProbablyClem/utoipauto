extern crate proc_macro;

use attribute_utils::update_openapi_macro_attributes;
use proc_macro::TokenStream;

use quote::quote;
use string_utils::{discover_paths, extract_paths};
use syn::parse_macro_input;
use token_utils::{check_macro_placement, extract_attributes, output_macro};
use utoipa_auto_core::{attribute_utils, string_utils, token_utils};

/// Macro to automatically discover all the functions with the #[utoipa] attribute
#[proc_macro_attribute]
pub fn utoipa_auto_discovery(
    attributes: proc_macro::TokenStream, // #[utoipa_auto_discovery(paths = "(MODULE_TREE_PATH => MODULE_SRC_PATH) ;")]
    item: proc_macro::TokenStream,       // #[openapi(paths = "")]
) -> proc_macro::TokenStream {
    // (MODULE_TREE_PATH => MODULE_SRC_PATH) ; (MODULE_TREE_PATH => MODULE_SRC_PATH) ; ...
    let paths: String = extract_attributes(attributes);
    // [(MODULE_TREE_PATH, MODULE_SRC_PATH)]
    let paths: Vec<String> = extract_paths(paths);

    // #[openapi(...)]
    let mut openapi_macro = parse_macro_input!(item as syn::ItemStruct);

    // Discover all the functions with the #[utoipa] attribute
    let (uto_paths, uto_models, uto_reponses): (String, String, String) = discover_paths(paths);

    // extract the openapi macro attributes : #[openapi(openapi_macro_attibutes)]
    let openapi_macro_attibutes = &mut openapi_macro.attrs;

    // Check if the macro is placed before the #[derive] and #[openapi] attributes
    check_macro_placement(openapi_macro_attibutes.clone());

    // Update the openapi macro attributes with the newly discovered paths
    update_openapi_macro_attributes(
        openapi_macro_attibutes,
        &uto_paths,
        &uto_models,
        &uto_reponses,
    );

    // Output the macro back to the compiler
    output_macro(openapi_macro)
}

/// Ignore the function from the auto discovery
#[proc_macro_attribute]
pub fn utoipa_ignore(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    let code = quote!(
          #input
    );

    TokenStream::from(code)
}
