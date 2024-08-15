use std::{
    fs::{self, File},
    io::{self, Read},
    iter,
    path::{Path, PathBuf},
};

pub fn parse_file<T: Into<PathBuf>>(filepath: T) -> Result<syn::File, io::Error> {
    let pb: PathBuf = filepath.into();

    if !pb.is_file() {
        panic!("File not found: {:?}", pb);
    }

    let mut file = File::open(&pb)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(syn::parse_file(&content).unwrap_or_else(move |_| panic!("Failed to parse file {:?}", pb)))
}

/// Parse all the files in the given path
pub fn parse_files<T: Into<PathBuf>>(path: T) -> Result<Vec<(String, syn::File)>, io::Error> {
    let mut files: Vec<(String, syn::File)> = vec![];

    let pb: PathBuf = path.into();
    if pb.is_file() {
        // we only parse rust files
        if is_rust_file(&pb) {
            files.push((pb.to_str().unwrap().to_string(), parse_file(pb)?));
        }
    } else {
        for entry in fs::read_dir(pb)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && is_rust_file(&path) {
                files.push((path.to_str().unwrap().to_string(), parse_file(path)?));
            } else {
                files.append(&mut parse_files(path)?);
            }
        }
    }
    Ok(files)
}

fn is_rust_file(path: &Path) -> bool {
    path.is_file()
        && match path.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => ext.eq("rs"),
                None => false,
            },
            None => false,
        }
}

/// Extract the module name from the file path
/// # Example
/// ```
/// use utoipauto_core::file_utils::extract_module_name_from_path;
/// let module_name = extract_module_name_from_path(
///    &"./utoipa-auto-macro/tests/controllers/controller1.rs".to_string(),
/// "crate"
/// );
/// assert_eq!(
///  module_name,
/// "crate::controllers::controller1".to_string()
/// );
/// ```
pub fn extract_module_name_from_path(path: &str, crate_name: &str) -> String {
    let path = path.replace('\\', "/");
    let path = path
        .trim_end_matches(".rs")
        .trim_end_matches("/mod")
        .trim_end_matches("/lib")
        .trim_end_matches("/main")
        .trim_start_matches("./");
    let segments: Vec<_> = path.split('/').collect();

    // In general, paths will look like `./src/my/module`, which should turn into `crate::my::module`.
    // When using cargo workspaces, paths may look like `./subcrate/src/my/module`,
    // `./crates/subcrate/src/my/module`, etc., so we need to remove anything up to `src`
    // (or `tests`) to still produce `crate::my::module`.
    // So we split the segments by the last occurrence of `src` or `tests` and take the last part.
    let segments_inside_crate = find_segment_and_skip(&segments, &["src", "tests"], 1);

    // Also skip fragments that are already out of the crate name. For example,
    // `./src/lib/my/module/name from crate::my::module` should turn into `crate::my::module:name`,
    // and not into `crate::lib::my::module::name`.
    let crate_name = crate_name.replace("-", "_");
    let mut crate_segments = crate_name.split("::");
    let first_crate_fragment = crate_segments.next().expect("Crate should not be empty");
    let segments_inside_crate = match crate_segments.next() {
        Some(crate_fragment) => find_segment_and_skip(segments_inside_crate, &[crate_fragment], 0),
        None => segments_inside_crate,
    };

    let full_crate_path: Vec<_> = iter::once(first_crate_fragment)
        .chain(segments_inside_crate.iter().copied())
        .collect();
    full_crate_path.join("::")
}

fn find_segment_and_skip<'a>(segments: &'a [&str], to_find: &[&str], to_skip: usize) -> &'a [&'a str] {
    match segments.iter().rposition(|segment| to_find.contains(segment)) {
        Some(idx) => &segments[(idx + to_skip)..],
        None => segments,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_module_name_from_path() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/controller1.rs", "crate"),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_path_windows() {
        assert_eq!(
            extract_module_name_from_path(".\\utoipa-auto-macro\\tests\\controllers\\controller1.rs", "crate"),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_mod() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/mod.rs", "crate"),
            "crate::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_lib() {
        assert_eq!(extract_module_name_from_path("./src/lib.rs", "crate"), "crate");
    }

    #[test]
    fn test_extract_module_name_from_main() {
        assert_eq!(extract_module_name_from_path("./src/main.rs", "crate"), "crate");
    }

    #[test]
    fn test_extract_module_name_from_workspace() {
        assert_eq!(
            extract_module_name_from_path("./server/src/routes/asset.rs", "crate"),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_nested() {
        assert_eq!(
            extract_module_name_from_path("./crates/server/src/routes/asset.rs", "crate"),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders() {
        assert_eq!(
            extract_module_name_from_path("./src/routing/api/audio.rs", "crate"),
            "crate::routing::api::audio"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders_nested() {
        assert_eq!(
            extract_module_name_from_path("./src/applications/src/retail_api/controllers/mod.rs", "crate"),
            "crate::retail_api::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders_nested_external_crate() {
        assert_eq!(
            extract_module_name_from_path("./src/applications/src/retail_api/controllers/mod.rs", "other_crate"),
            "other_crate::retail_api::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_with_prefix_path() {
        assert_eq!(
            extract_module_name_from_path("./crates/server/src/routes_lib/routes/asset.rs", "crate::routes"),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_with_external_crate_and_underscore() {
        assert_eq!(
            extract_module_name_from_path(
                "./src/applications/src/retail-api/controllers/mod.rs",
                "other-crate"
            ),
            "other_crate::retail-api::controllers"
        );
    }
}
