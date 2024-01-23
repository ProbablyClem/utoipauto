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
    path.is_file() && path.extension().unwrap().to_str().unwrap().eq("rs")
}

/// Extract the module name from the file path
/// # Example
/// ```
/// use utoipauto_core::file_utils::extract_module_name_from_path;
/// let module_name = extract_module_name_from_path(
///    &"./utoipa-auto-macro/tests/controllers/controller1.rs".to_string()
/// );
/// assert_eq!(
///  module_name,
/// "crate::controllers::controller1".to_string()
/// );
/// ```
pub fn extract_module_name_from_path(path: &str) -> String {
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
    let segments_inside_crate = match segments
        .iter()
        .position(|&segment| segment == "src" || segment == "tests")
    {
        Some(idx) => &segments[(idx + 1)..],
        None => &segments,
    };
    let full_crate_path: Vec<_> = iter::once("crate")
        .chain(segments_inside_crate.iter().copied())
        .collect();
    full_crate_path.join("::")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_module_name_from_path() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/controller1.rs"),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_path_windows() {
        assert_eq!(
            extract_module_name_from_path(
                ".\\utoipa-auto-macro\\tests\\controllers\\controller1.rs"
            ),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_mod() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/mod.rs"),
            "crate::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_lib() {
        assert_eq!(extract_module_name_from_path("./src/lib.rs"), "crate");
    }

    #[test]
    fn test_extract_module_name_from_main() {
        assert_eq!(extract_module_name_from_path("./src/main.rs"), "crate");
    }

    #[test]
    fn test_extract_module_name_from_workspace() {
        assert_eq!(
            extract_module_name_from_path("./server/src/routes/asset.rs"),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_nested() {
        assert_eq!(
            extract_module_name_from_path("./crates/server/src/routes/asset.rs"),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders() {
        assert_eq!(
            extract_module_name_from_path("./src/routing/api/audio.rs"),
            "crate::routing::api::audio"
        );
    }
}
