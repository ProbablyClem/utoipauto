use crate::discover::discover_from_file;

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

/// Extract the file paths from the attributes
/// Support the old syntax (MODULE_TREE_PATH => MODULE_SRC_PATH) ; (MODULE_TREE_PATH => MODULE_SRC_PATH) ;
/// and the new syntax MODULE_SRC_PATH, MODULE_SRC_PATH
///
/// # Example
/// ```
/// use utoipauto_core::string_utils::extract_paths;
/// let paths = extract_paths(
///    "\"(utoipa_auto_macro::tests::controllers::controller1 => ./utoipa-auto-macro/tests/controllers/controller1.rs) ; (utoipa_auto_macro::tests::controllers::controller2 => ./utoipa-auto-macro/tests/controllers/controller2.rs)\""
///       .to_string()
/// );
/// assert_eq!(
///   paths,
///  vec![
///    "./utoipa-auto-macro/tests/controllers/controller1.rs".to_string(),
///   "./utoipa-auto-macro/tests/controllers/controller2.rs".to_string()
/// ]   
/// );
/// ```
pub fn extract_paths(attributes: String) -> Vec<String> {
    let attributes = trim_parentheses(rem_first_and_last(&attributes));

    if attributes.contains('|') {
        panic!("Please use the new syntax ! paths=\"(MODULE_TREE_PATH => MODULE_SRC_PATH) ;\"")
    }
    let paths = if attributes.contains("=>") {
        extract_paths_arrow(attributes)
    } else {
        extract_paths_coma(attributes)
    };
    if paths.is_empty() {
        panic!("utoipauto: No paths specified !")
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

/// Return the list of all the functions with the #[utoipa] attribute
/// and the list of all the structs with the #[derive(ToSchema)] attribute
/// and the list of all the structs with the #[derive(ToResponse)] attribute
pub fn discover(paths: Vec<String>) -> (String, String, String) {
    let mut uto_paths: String = String::new();
    let mut uto_models: String = String::new();
    let mut uto_reponses: String = String::new();
    for p in paths {
        let path = extract_crate_name(p);
        let (list_fn, list_model, list_reponse) = discover_from_file(path.paths, path.crate_name);
        // We need to add a coma after each path
        for i in list_fn {
            uto_paths.push_str(format!("{},", i).as_str());
        }
        for i in list_model {
            uto_models.push_str(format!("{},", i).as_str());
        }
        for i in list_reponse {
            uto_reponses.push_str(format!("{},", i).as_str());
        }
    }
    (uto_paths, uto_models, uto_reponses)
}

#[derive(Debug, PartialEq)]
struct Path {
    paths: String,
    crate_name: String,
}

fn extract_crate_name(path: String) -> Path {
    let mut path = path.split(" from ");
    let paths = path.next().unwrap();
    let crate_name = path.next().unwrap_or("crate").to_string();
    Path {
        paths: paths.to_string(),
        crate_name,
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_extract_crate_name() {
        assert_eq!(
            super::extract_crate_name(
                "utoipa_auto_macro::from::controllers::controller1 from utoipa_auto_macro"
                    .to_string(),
            ),
            super::Path {
                paths: "utoipa_auto_macro::from::controllers::controller1".to_string(),
                crate_name: "utoipa_auto_macro".to_string()
            }
        );
    }

    #[test]
    fn test_extract_crate_name_default() {
        assert_eq!(
            super::extract_crate_name(
                "utoipa_auto_macro::from::controllers::controller1".to_string()
            ),
            super::Path {
                paths: "utoipa_auto_macro::from::controllers::controller1".to_string(),
                crate_name: "crate".to_string()
            }
        );
    }

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
