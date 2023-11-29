use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;
/// Extract the paths string attribute from the proc_macro::TokenStream
pub fn extract_attributes(stream: proc_macro::TokenStream) -> String {
    let mut paths: String = "".to_string();
    let mut has_paths = false;
    if !stream.is_empty() {
        let mut it = stream.into_iter();
        let tok = it.next();

        if let Some(proc_macro::TokenTree::Ident(ident)) = tok {
            if ident.to_string().eq("paths") {
                has_paths = true;
                let tok = it.next();
                if let Some(proc_macro::TokenTree::Punct(punct)) = tok {
                    if punct.to_string().eq("=") {
                        let tok = it.next();
                        if let Some(tok) = tok {
                            match tok {
                                proc_macro::TokenTree::Literal(lit) => {
                                    paths = lit.to_string();
                                }
                                _ => {
                                    panic!("malformed paths !")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    // if no paths specified, we use the default path "./src"
    if !has_paths {
        "\"./src\"".to_string()
    } else {
        paths
    }
}

/// Check if the macro is placed before the #[derive] and #[openapi] attributes
/// Otherwise, panic!
pub fn check_macro_placement(attrs: Vec<Attribute>) {
    if !attrs.iter().any(|elm| elm.path().is_ident("derive")) {
        panic!("Please put utoipa_auto_discovery before #[derive] and #[openapi]");
    }

    if !attrs.iter().any(|elm| elm.path().is_ident("openapi")) {
        panic!("Please put utoipa_auto_discovery before #[derive] and #[openapi]");
    }
}

// Output the macro back to the compiler
pub fn output_macro(openapi_macro: syn::ItemStruct) -> proc_macro::TokenStream {
    let code = quote!(
          #openapi_macro
    );

    TokenStream::from(code)
}
