use std::vec;

use syn::{punctuated::Punctuated, ItemFn, ItemMod, ItemStruct, Meta, Token};

use crate::file_utils::{extract_module_name_from_path, parse_files};
// refactoring the previous methodes by iterating instead of recursing
pub fn get_all_uto_functions_iter(src_path: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut fns_name: Vec<String> = vec![];
    let mut models_name: Vec<String> = vec![];
    let mut responses_name: Vec<String> = vec![];
    let files =
        parse_files(&src_path).unwrap_or_else(|_| panic!("Failed to parse file {}", src_path));

    for file in files {
        let filename = file.0;
        let file = file.1;
        for i in file.items {
            let fns = match &i {
                syn::Item::Mod(m) => parse_module_fns(&m),
                syn::Item::Fn(f) => parse_function(&f),
                _ => vec![],
            };
            let (models, reponses) = match i {
                syn::Item::Mod(m) => parse_module_structs(&m),
                syn::Item::Struct(s) => parse_struct(&s),
                _ => (vec![], vec![]),
            };
            for fn_name in fns {
                fns_name.push(build_path(&filename, &fn_name));
            }
            for model_name in models {
                models_name.push(build_path(&filename, &model_name));
            }
            for response_name in reponses {
                responses_name.push(build_path(&filename, &response_name));
            }
        }
    }
    (fns_name, models_name, responses_name)
}

fn build_path(file_name: &String, fn_name: &String) -> String {
    format!("{}::{}", extract_module_name_from_path(file_name), fn_name)
}

/// Search for ToSchema and ToResponse implementations
fn parse_struct(t: &ItemStruct) -> (Vec<String>, Vec<String>) {
    let mut models_name: Vec<String> = vec![];
    let mut responses_name: Vec<String> = vec![];
    let attrs = &t.attrs;
    for attr in attrs {
        let meta = &attr.meta;
        if meta.path().is_ident("derive") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for nested_meta in nested {
                if nested_meta.path().is_ident("ToSchema") {
                    models_name.push(t.ident.to_string());
                }
                if nested_meta.path().is_ident("ToResponse") {
                    responses_name.push(t.ident.to_string());
                }
            }
        }
    }

    (models_name, responses_name)
}

fn parse_module_fns(m: &ItemMod) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    if let Some((_, items)) = &m.content {
        for it in items {
            match it {
                syn::Item::Mod(m) => fns_name.append(&mut parse_module_fns(m)),
                syn::Item::Fn(f) => fns_name.append(
                    &mut parse_function(f)
                        .into_iter()
                        .map(|item| format!("{}::{}", m.ident, item))
                        .collect::<Vec<String>>(),
                ),

                _ => {}
            }
        }
    }
    fns_name
}

fn parse_module_structs(m: &ItemMod) -> (Vec<String>, Vec<String>) {
    let mut models_name: Vec<String> = vec![];
    let mut responses_name: Vec<String> = vec![];
    if let Some((_, items)) = &m.content {
        for it in items {
            let (models, reponses) = match it {
                syn::Item::Mod(m) => parse_module_structs(m),
                syn::Item::Struct(s) => parse_struct(s),

                _ => (vec![], vec![]),
            };
            for model_name in models {
                models_name.push(format!("{}::{}", m.ident, model_name));
            }
            for response_name in reponses {
                responses_name.push(format!("{}::{}", m.ident, response_name));
            }
        }
    }
    (models_name, responses_name)
}

fn parse_function(f: &ItemFn) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    if should_parse_fn(f) {
        for i in 0..f.attrs.len() {
            if f.attrs[i]
                .meta
                .path()
                .segments
                .iter()
                .any(|item| item.ident.eq("utoipa"))
            {
                fns_name.push(f.sig.ident.to_string());
            }
        }
    }
    fns_name
}

fn should_parse_fn(f: &ItemFn) -> bool {
    !f.attrs.is_empty() && !is_fn_ignored(f)
}

fn is_fn_ignored(f: &ItemFn) -> bool {
    f.attrs.iter().any(|attr| {
        if let Some(name) = attr.path().get_ident() {
            name.eq("utoipa_ignore")
        } else {
            false
        }
    })
}
