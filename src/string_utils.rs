use crate::discover::get_all_uto_functions;

pub fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

pub fn trim_whites(str: &str) -> String {
    let s = str.trim();

    let s: String = s.replace('\n', "");

    s
}

pub fn trim_parentheses(str: &str) -> String {
    let s = str.trim();

    let s: String = s.replace(['(', ')'], "");

    s
}

pub fn extract_pairs(paths: String) -> Vec<(String, String)> {
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

    if pairs.is_empty() {
        panic!("utoipa_auto_discovery: No paths specified !")
    }
    pairs
}

pub fn discover_paths(pairs: Vec<(String, String)>) -> String {
    let mut uto_paths: String = String::new();
    for p in pairs {
        let list_fn = get_all_uto_functions(p.1);

        if !list_fn.is_empty() {
            for i in list_fn {
                uto_paths.push_str(format!("{}::{},", p.0, i).as_str());
            }
        }
    }
    uto_paths
}
