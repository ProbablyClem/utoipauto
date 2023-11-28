use syn::{Item, ItemFn, ItemMod};

use crate::file_utils::parse_files;

pub fn get_all_mod_uto_functions(item: &syn::ItemMod, fns_name: &mut Vec<String>) {
    // if item.content.is_none() {
    //     return;
    // }

    let next_items = &item.content.iter().next();
    let sub_items = match next_items {
        Some(next) => &next.1,
        None => return,
    };
    let mod_name = item.ident.to_string();

    for it in sub_items {
        match it {
            syn::Item::Mod(m) => get_all_mod_uto_functions(m, fns_name),
            syn::Item::Fn(f) => {
                if !f.attrs.is_empty()
                    && !f.attrs.iter().any(|attr| {
                        if let Some(name) = attr.path().get_ident() {
                            name.eq("utoipa_ignore")
                        } else {
                            false
                        }
                    })
                {
                    for i in 0..f.attrs.len() {
                        if f.attrs[i]
                            .meta
                            .path()
                            .segments
                            .iter()
                            .any(|item| item.ident.eq("utoipa"))
                        {
                            fns_name.push(format!("{}::{}", mod_name, f.sig.ident));
                        }
                    }
                }
            }

            _ => {}
        }
    }
}

pub fn get_all_uto_functions(src_path: String) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];

    let files =
        parse_files(&src_path).unwrap_or_else(|_| panic!("Failed to parse file {}", src_path));
    let items = files
        .into_iter()
        .flat_map(|sc| sc.items)
        .collect::<Vec<Item>>();

    for i in items {
        match i {
            syn::Item::Mod(m) => get_all_mod_uto_functions(&m, &mut fns_name),
            syn::Item::Fn(f) => {
                if !f.attrs.is_empty()
                    && !f.attrs.iter().any(|attr| {
                        if let Some(name) = attr.path().get_ident() {
                            name.eq("utoipa_ignore")
                        } else {
                            false
                        }
                    })
                {
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
            }

            _ => {}
        }
    }

    fns_name
}

// refactoring the previous methodes by iterating instead of recursing
pub fn get_all_uto_functions_iter(src_path: String) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];

    let files =
        parse_files(&src_path).unwrap_or_else(|_| panic!("Failed to parse file {}", src_path));
    let items = files
        .into_iter()
        .flat_map(|sc| sc.items)
        .collect::<Vec<Item>>();

    for i in items {
        match i {
            syn::Item::Mod(m) => fns_name.append(&mut parse_module(&m)),
            syn::Item::Fn(f) => fns_name.append(&mut parse_function(&f)),
            _ => {}
        }
    }
    fns_name
}

fn parse_module(m: &ItemMod) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];
    if let Some((_, items)) = &m.content {
        for it in items {
            match it {
                syn::Item::Mod(m) => fns_name.append(&mut parse_module(m)),
                syn::Item::Fn(f) => fns_name.append(&mut parse_function(f)),

                _ => {}
            }
        }
    }
    fns_name
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
