use crate::discover::get_all_uto_functions_iter;

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

pub fn extract_paths(attributes: String) -> Vec<String> {
    let paths;
    println!("attributes: {:?}", attributes);
    let attributes = trim_parentheses(rem_first_and_last(&attributes.as_str()));

    if attributes.contains('|') {
        panic!("Please use the new syntax ! paths=\"(MODULE_TREE_PATH => MODULE_SRC_PATH) ;\"")
    }
    if attributes.contains("=>") {
        paths = extract_paths_arrow(attributes);
    } else {
        paths = extract_paths_coma(attributes);
    }
    if paths.is_empty() {
        panic!("utoipa_auto_discovery: No paths specified !")
    }
    paths
}

// (MODULE_TREE_PATH => MODULE_SRC_PATH) ; (MODULE_TREE_PATH => MODULE_SRC_PATH) ;
// extract the paths from the string
// Here for legacy support
fn extract_paths_arrow(attributes: String) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let attributes = attributes.split(';');

    for p in attributes {
        let pair = p.split_once("=>");

        if let Some(pair) = pair {
            paths.push(trim_whites(pair.1));
        }
    }
    paths
}

// MODULE_SRC_PATH, MODULE_SRC_PATH
fn extract_paths_coma(attributes: String) -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let attributes = attributes.split(',');

    for p in attributes {
        paths.push(trim_whites(p));
    }
    paths
}

pub fn discover_paths(paths: Vec<String>) -> String {
    let mut uto_paths: String = String::new();
    for p in paths {
        let list_fn = get_all_uto_functions_iter(p);
        println!("list_fn: {:?}", list_fn);

        for i in list_fn {
            // uto_paths.push_str(format!("{}::{},", p.0, i).as_str());
            uto_paths.push_str(format!("{},", i).as_str());
        }
    }
    println!("uto_paths: {:?}", uto_paths);
    uto_paths
}

#[cfg(test)]
mod test {
    #[test]
    fn test_extract_paths_arrow() {
        assert_eq!(
            super::extract_paths(
                "\"(utoipa_auto_macro::tests::controllers::controller1 => ./utoipa-auto-macro/tests/controllers/controller1.rs) ; (utoipa_auto_macro::tests::controllers::controller2 => ./utoipa-auto-macro/tests/controllers/controller2.rs)\""
                    .to_string()
            ),
            vec![
                "./utoipa-auto-macro/tests/controllers/controller1.rs".to_string(),
                "./utoipa-auto-macro/tests/controllers/controller2.rs".to_string()
            ]
        );
    }

    #[test]
    fn test_extract_paths_coma() {
        assert_eq!(
            super::extract_paths(
                "\"./utoipa-auto-macro/tests/controllers/controller1.rs, ./utoipa-auto-macro/tests/controllers/controller2.rs\""
                    .to_string()
            ),
            vec![
                "./utoipa-auto-macro/tests/controllers/controller1.rs".to_string(),
                "./utoipa-auto-macro/tests/controllers/controller2.rs".to_string()
            ]
        );
    }
}
