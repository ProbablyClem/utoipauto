use crate::file_utils::parse_file;

pub fn get_all_mod_uto_functions(item: &syn::ItemMod, fns_name: &mut Vec<String>) {
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

pub fn get_all_uto_functions(src_path: String) -> Vec<String> {
    let mut fns_name: Vec<String> = vec![];

    let sc = parse_file(&src_path).expect(format!("Failed to parse file {}", src_path).as_str());
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

    fns_name
}
