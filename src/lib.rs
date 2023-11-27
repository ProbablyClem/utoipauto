extern crate proc_macro;
mod file_utils;
mod string_utils;

use file_utils::parse_file;
use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use string_utils::{rem_first_and_last, trim_parentheses, trim_whites};
use syn::parse_macro_input;

fn get_all_mod_uto_functions(item: &syn::ItemMod, fns_name: &mut Vec<String>) {
    let sub_items = &item.content.iter().next().unwrap().1;

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

fn get_all_uto_functions(src_path: String) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];

    let sc = parse_file(&src_path);
    if let Ok(sc) = sc {
        let items = sc.items;

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
    }

    fns_name
}

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

#[proc_macro_attribute]
pub fn utoipa_auto_discovery(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemStruct);

    let mut paths: String = "".to_string();

    if !attr.is_empty() {
        let mut it = attr.into_iter();
        let tok = it.next();

        if let Some(proc_macro::TokenTree::Ident(ident)) = tok {
            if ident.to_string().eq("paths") {
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

    let mut pairs: Vec<(String, String)> = vec![];

    let str_paths = trim_parentheses(rem_first_and_last(paths.as_str()));

    if str_paths.contains('|') {
        panic!("Please use the new syntax ! paths=\"(MODULE_TREE_PATH => MODULE_SRC_PATH) ;\"")
    }

    let paths = str_paths.split(';');

    for p in paths {
        let pair = p.split_once("=>");

        if let Some(pair) = pair {
            pairs.push((trim_whites(pair.0), trim_whites(pair.1)));
        }
    }

    if !pairs.is_empty() {
        let mut uto_paths: String = String::new();

        for p in pairs {
            let list_fn = get_all_uto_functions(p.1);

            if !list_fn.is_empty() {
                for i in list_fn {
                    uto_paths.push_str(format!("{}::{},", p.0, i).as_str());
                }
            }
        }

        let attrs = &mut input.attrs;

        if !attrs.iter().any(|elm| elm.path().is_ident("derive")) {
            panic!("Please put utoipa_auto_discovery before #[derive] and #[openapi]");
        }

        if !attrs.iter().any(|elm| elm.path().is_ident("openapi")) {
            panic!("Please put utoipa_auto_discovery before #[derive] and #[openapi]");
        }

        let mut is_ok: bool = false;
        #[warn(clippy::needless_range_loop)]
        for i in 0..attrs.len() {
            if attrs[i].path().is_ident("openapi") {
                is_ok = true;
                let mut src_uto_macro = attrs[i].to_token_stream().to_string();

                src_uto_macro = src_uto_macro.replace("#[openapi(", "");
                src_uto_macro = src_uto_macro.replace(")]", "");

                if !src_uto_macro.contains("paths(") {
                    let new_paths = format!("paths({}", uto_paths);
                    src_uto_macro = format!("{}), {}", new_paths, src_uto_macro);

                    let stream: proc_macro2::TokenStream = src_uto_macro.parse().unwrap();

                    let new_attr: syn::Attribute = syn::parse_quote! { #[openapi( #stream )] };

                    attrs[i] = new_attr;
                } else {
                    let new_paths = format!("paths({}", uto_paths);

                    src_uto_macro = src_uto_macro.replace("paths(", new_paths.as_str());

                    let stream: proc_macro2::TokenStream = src_uto_macro.parse().unwrap();

                    let new_attr: syn::Attribute = syn::parse_quote! { #[openapi( #stream )] };

                    attrs[i] = new_attr;
                }
            }
        }

        if !is_ok {
            panic!("No utoipa::openapi Macro found !");
        }
    }

    let code = quote!(
          #input
    );

    TokenStream::from(code)
}
