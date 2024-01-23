use std::vec;

use quote::ToTokens;
use syn::{punctuated::Punctuated, Attribute, Item, ItemFn, Meta, Token};

use crate::file_utils::{extract_module_name_from_path, parse_files};

/// Discover everything from a file, will explore folder recursively
pub fn discover_from_file(src_path: String) -> (Vec<String>, Vec<String>, Vec<String>) {
    let files =
        parse_files(&src_path).unwrap_or_else(|_| panic!("Failed to parse file {}", src_path));

    files
        .into_iter()
        .map(|e| parse_module_items(&extract_module_name_from_path(&e.0), e.1.items))
        .fold(Vec::<DiscoverType>::new(), |mut acc, mut v| {
            acc.append(&mut v);
            acc
        })
        .into_iter()
        .fold(
            (
                Vec::<String>::new(),
                Vec::<String>::new(),
                Vec::<String>::new(),
            ),
            |mut acc, v| {
                match v {
                    DiscoverType::Fn(n) => acc.0.push(n),
                    DiscoverType::Model(n) => acc.1.push(n),
                    DiscoverType::Response(n) => acc.2.push(n),
                    DiscoverType::CustomImpl(n) => acc.1.push(n),
                };

                acc
            },
        )
}

enum DiscoverType {
    Fn(String),
    Model(String),
    Response(String),
    CustomImpl(String),
}

fn parse_module_items(module_path: &str, items: Vec<Item>) -> Vec<DiscoverType> {
    items
        .into_iter()
        .filter(|e| {
            matches!(
                e,
                syn::Item::Mod(_)
                    | syn::Item::Fn(_)
                    | syn::Item::Struct(_)
                    | syn::Item::Enum(_)
                    | syn::Item::Impl(_)
            )
        })
        .map(|v| match v {
            syn::Item::Mod(m) => m.content.map_or(Vec::<DiscoverType>::new(), |cs| {
                parse_module_items(&build_path(module_path, &m.ident.to_string()), cs.1)
            }),
            syn::Item::Fn(f) => parse_function(&f)
                .into_iter()
                .map(|item| DiscoverType::Fn(build_path(module_path, &item)))
                .collect(),
            syn::Item::Struct(s) => {
                parse_from_attr(&s.attrs, &build_path(module_path, &s.ident.to_string()))
            }
            syn::Item::Enum(e) => {
                parse_from_attr(&e.attrs, &build_path(module_path, &e.ident.to_string()))
            }
            syn::Item::Impl(im) => {
                parse_from_impl(&im, module_path)
            },
            _ => vec![],
        })
        .fold(Vec::<DiscoverType>::new(), |mut acc, mut v| {
            acc.append(&mut v);
            acc
        })
}

/// Search for ToSchema and ToResponse implementations in attr
fn parse_from_attr(a: &Vec<Attribute>, name: &str) -> Vec<DiscoverType> {
    let mut out: Vec<DiscoverType> = vec![];

    for attr in a {
        let meta = &attr.meta;
        if meta.path().is_ident("utoipa_ignore") {
            return vec![];
        }
        if meta.path().is_ident("derive") {
            let nested = attr
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();
            for nested_meta in nested {
                if nested_meta.path().is_ident("ToSchema") {
                    out.push(DiscoverType::Model(name.to_string()));
                }
                if nested_meta.path().is_ident("ToResponse") {
                    out.push(DiscoverType::Response(name.to_string()));
                }
            }
        }
    }

    out
}

fn parse_from_impl(im: &syn::ItemImpl, module_base_path: &str) -> Vec<DiscoverType> {
    im.trait_
        .as_ref()
        .and_then(|trt| trt.1.segments.last().cloned())
        .and_then(|tname| {
            tname
                .to_token_stream()
                .to_string()
                .starts_with("ToSchema")
                .then_some(im.self_ty.to_token_stream().to_string())
        })
        .map(|valid_impl| {
            vec![DiscoverType::CustomImpl(build_path(
                module_base_path,
                &valid_impl,
            ))]
        })
        .unwrap_or(vec![])
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
    !f.attrs.is_empty() && !is_ignored(f)
}

fn is_ignored(f: &ItemFn) -> bool {
    f.attrs.iter().any(|attr| {
        if let Some(name) = attr.path().get_ident() {
            name.eq("utoipa_ignore")
        } else {
            false
        }
    })
}

fn build_path(file_name: &str, fn_name: &str) -> String {
    format!("{}::{}", file_name, fn_name)
}
