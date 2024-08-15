use attribute_utils::update_openapi_macro_attributes;
use proc_macro::TokenStream;

use quote::quote;
use string_utils::{discover, extract_paths};
use syn::parse_macro_input;
use token_utils::{check_macro_placement, extract_attributes, output_macro};
use utoipauto_core::{attribute_utils, string_utils, token_utils};

/// Macro to automatically discover all the functions with the #[utoipa] attribute
/// And the struct deriving ToSchema and ToResponse
#[proc_macro_attribute]
pub fn utoipauto(
    attributes: proc_macro::TokenStream, // #[utoipauto(paths = "(MODULE_TREE_PATH => MODULE_SRC_PATH) ;")]
    item: proc_macro::TokenStream,       // #[openapi(paths = "")]
) -> proc_macro::TokenStream {
    // (MODULE_TREE_PATH => MODULE_SRC_PATH) ; (MODULE_TREE_PATH => MODULE_SRC_PATH) ; ...
    let params = extract_attributes(attributes.into());
    // [(MODULE_TREE_PATH, MODULE_SRC_PATH)]
    let paths: Vec<String> = extract_paths(&params.paths);

    // #[openapi(...)]
    let mut openapi_macro = parse_macro_input!(item as syn::ItemStruct);

    // Discover all the functions with the #[utoipa] attribute
    let (uto_paths, uto_models, uto_responses): (String, String, String) = discover(paths, &params);

    // extract the openapi macro attributes : #[openapi(openapi_macro_attibutes)]
    let openapi_macro_attibutes = &mut openapi_macro.attrs;

    // Check if the macro is placed before the #[derive] and #[openapi] attributes
    check_macro_placement(openapi_macro_attibutes.clone());

    // Update the openapi macro attributes with the newly discovered paths
    update_openapi_macro_attributes(openapi_macro_attibutes, &uto_paths, &uto_models, &uto_responses);

    // Output the macro back to the compiler
    output_macro(openapi_macro)
}

/// Ignore the function from the auto discovery
#[proc_macro_attribute]
pub fn utoipa_ignore(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::Item);
    let code = quote!(
          #input
    );

    TokenStream::from(code)
}

/// Useless macro to test custom function attributes
#[proc_macro_attribute]
pub fn test_handler(_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as syn::ItemFn);

    let code = quote!(
        #[utoipa::path(get, path = "/")]
        #input
    );

    TokenStream::from(code)
}
