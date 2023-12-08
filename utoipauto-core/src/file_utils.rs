use std::{
    fs::{self, File},
    io::{self, Read},
    path::PathBuf,
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

fn is_rust_file(path: &PathBuf) -> bool {
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
/// "crate::tests::controllers::controller1".to_string()
/// );
/// ```
pub fn extract_module_name_from_path(path: &String) -> String {
    let mut path = path.to_string();
    if path.ends_with(".rs") {
        path = path.replace(".rs", "");
    }
    if path.ends_with("/mod") {
        path = path.replace("/mod", "");
    }
    if path.ends_with("/lib") {
        path = path.replace("/lib", "");
    }
    if path.ends_with("/main") {
        path = path.replace("/main", "");
    }
    path = path.replace("./", "");
    //remove first word
    let path_vec = path
        .split('/')
        .enumerate()
        .filter_map(|(idx, segment)| match (idx, segment) {
            // Remove first segment and replace with `crate`, so that 'root/thing' becomes
            // `crate::thing` in the end
            (0, _) => Some("crate"),
            // When using cargo workspaces, paths look like  './subcrate/src/my/module',
            // so we need to remove the 'src' to produce `crate::my::module`
            (1, "src") => None,
            (_, segment) => Some(segment),
        })
        .collect::<Vec<&str>>();
    path_vec.join("::")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_module_name_from_path() {
        assert_eq!(
            extract_module_name_from_path(
                &"./utoipa-auto-macro/tests/controllers/controller1.rs".to_string()
            ),
            "crate::tests::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_mod() {
        assert_eq!(
            extract_module_name_from_path(
                &"./utoipa-auto-macro/tests/controllers/mod.rs".to_string()
            ),
            "crate::tests::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_lib() {
        assert_eq!(
            extract_module_name_from_path(&"./src/lib.rs".to_string()),
            "crate"
        );
    }

    #[test]
    fn test_extract_module_name_from_main() {
        assert_eq!(
            extract_module_name_from_path(&"./src/main.rs".to_string()),
            "crate"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace() {
        assert_eq!(
            extract_module_name_from_path(&"./server/src/routes/asset.rs".to_string()),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders() {
        assert_eq!(
            extract_module_name_from_path(&"./src/routing/api/audio.rs".to_string()),
            "crate::routing::api::audio"
        );
    }
}
