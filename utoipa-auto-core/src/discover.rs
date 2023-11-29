use syn::{ItemFn, ItemMod};

use crate::file_utils::{extract_module_name_from_path, parse_files};
// refactoring the previous methodes by iterating instead of recursing
pub fn get_all_uto_functions_iter(src_path: String) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    println!("src_path: {:?}", src_path);
    let files =
        parse_files(&src_path).unwrap_or_else(|_| panic!("Failed to parse file {}", src_path));

    for file in files {
        println!("file: {:?}", file.0);
        let filename = file.0;
        let file = file.1;
        for i in file.items {
            let fn_names = match i {
                syn::Item::Mod(m) => parse_module(&m),
                syn::Item::Fn(f) => parse_function(&f),
                _ => vec![],
            };
            for fn_name in fn_names {
                fns_name.push(build_path(&filename, &fn_name));
            }
        }
    }
    println!("fns_name: {:?}", fns_name);
    fns_name
}

fn build_path(file_name: &String, fn_name: &String) -> String {
    format!("{}::{}", extract_module_name_from_path(file_name), fn_name)
}

fn parse_module(m: &ItemMod) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    if let Some((_, items)) = &m.content {
        println!("mod content len: {:?}", items.len());
        for it in items {
            match it {
                syn::Item::Mod(m) => fns_name.append(&mut parse_module(m)),
                syn::Item::Fn(f) => fns_name.append(
                    &mut parse_function(f)
                        .into_iter()
                        .map(|item| format!("{}::{}", m.ident, item))
                        .collect::<Vec<String>>(),
                ),

                _ => {}
            }
        }
    } else {
        println!("mod content is none");
    }
    fns_name
}

fn parse_function(f: &ItemFn) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    println!("fn: {:?}", f.sig.ident);
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
