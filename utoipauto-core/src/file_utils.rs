use std::{
    fs::{self, File},
    io::{self, Read},
    iter,
    path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};
use proc_macro2::Span;

pub fn parse_file<T: Into<PathBuf>>(filepath: T) -> Result<syn::File, io::Error> {
    let pb: PathBuf = filepath.into();

    if !pb.is_file() {
        panic!("File not found: {:?}", pb);
    }

    let mut file = File::open(&pb)?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read {}: {}", pb.display(), e)))?;

    Ok(syn::parse_file(&content).unwrap_or_else(move |_| panic!("Failed to parse file {:?}", pb)))
}

/// Parse all the files in the given path, skipping any whose path matches one
/// of the `excludes` glob patterns.
pub fn parse_files<T: Into<PathBuf>>(path: T, excludes: &GlobSet) -> Result<Vec<(String, syn::File)>, io::Error> {
    let mut files: Vec<(String, syn::File)> = vec![];

    let pb: PathBuf = path.into();
    if excludes.is_match(&pb) {
        return Ok(files);
    }

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
                if !excludes.is_match(&path) {
                    files.push((path.to_str().unwrap().to_string(), parse_file(path)?));
                }
            } else {
                files.append(&mut parse_files(path, excludes)?);
            }
        }
    }
    Ok(files)
}

/// Build a matcher from a list of glob patterns. Blank entries are ignored, so
/// an empty list yields a set that matches nothing and lets every file through.
/// An unparseable pattern is a mistake in the caller's attribute, so it aborts
/// expansion rather than silently widening the scan.
pub fn build_exclude_set<S: AsRef<str>>(patterns: &[S]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns.iter().map(|p| p.as_ref().trim()).filter(|p| !p.is_empty()) {
        let glob = Glob::new(pattern).unwrap_or_else(|e| panic!("Invalid exclude pattern {:?}: {}", pattern, e));
        builder.add(glob);
    }
    builder.build().expect("Failed to build exclude pattern set")
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
/// # use quote::ToTokens as _;
/// use utoipauto_core::file_utils::extract_module_name_from_path;
/// let module_name = extract_module_name_from_path(
///    &"./utoipa-auto-macro/tests/controllers/controller1.rs".to_string(),
/// "crate"
/// );
/// assert_eq!(
///  module_name.to_token_stream().to_string().replace(' ', ""),
/// "crate::controllers::controller1".to_string()
/// );
/// ```
pub fn extract_module_name_from_path(path: &str, crate_name: &str) -> syn::Path {
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

    let full_crate_path = iter::once(first_crate_fragment)
        .chain(segments_inside_crate.iter().copied())
        .map(|segment| syn::PathSegment::from(syn::Ident::new(&segment.replace('-', "_"), Span::mixed_site())));
    syn::Path {
        leading_colon: None,
        segments: full_crate_path.collect(),
    }
}

fn find_segment_and_skip<'a>(segments: &'a [&str], to_find: &[&str], to_skip: usize) -> &'a [&'a str] {
    match segments.iter().rposition(|segment| to_find.contains(segment)) {
        Some(idx) => &segments[(idx + to_skip)..],
        None => segments,
    }
}

#[cfg(test)]
mod tests {
    use quote::ToTokens;

    use super::*;

    use crate::token_utils::DEFAULT_EXCLUDE;

    /// Write a valid source file alongside an AppleDouble sidecar. The bytes are
    /// written directly rather than left to the OS, so the fixture reproduces the
    /// sidecar's binary, non-UTF-8 payload on every platform.
    fn fixture_with_sidecar(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        fs::write(dir.join("controller.rs"), "pub fn handler() {}\n").unwrap();
        fs::write(
            dir.join("._controller.rs"),
            [0x00, 0x05, 0x16, 0x07, 0x00, 0x02, 0xb0, 0xff],
        )
        .unwrap();
        dir
    }

    #[test]
    fn test_default_exclude_skips_apple_double_sidecars() {
        let dir = fixture_with_sidecar("utoipauto_exclude_default");

        let parsed = parse_files(&dir, &build_exclude_set(DEFAULT_EXCLUDE)).unwrap();
        assert_eq!(parsed.len(), 1);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_empty_exclude_disables_filtering() {
        let dir = fixture_with_sidecar("utoipauto_exclude_disabled");

        // With no exclude patterns the sidecar is handed to the parser and its
        // non-UTF-8 contents surface as a read error naming the file.
        let err = match parse_files(&dir, &build_exclude_set::<&str>(&[])) {
            Ok(_) => panic!("expected a read error for the sidecar"),
            Err(e) => e,
        };
        assert!(err.to_string().contains("._controller.rs"), "{err}");

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_custom_exclude_skips_matching_directory() {
        let dir = std::env::temp_dir().join("utoipauto_exclude_custom");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("generated")).unwrap();

        fs::write(dir.join("controller.rs"), "pub fn handler() {}\n").unwrap();
        fs::write(dir.join("generated").join("schema.rs"), "pub struct S;\n").unwrap();

        let parsed = parse_files(&dir, &build_exclude_set(&["**/generated/**"])).unwrap();
        assert_eq!(parsed.len(), 1);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_exclude_supports_alternate_globs() {
        let dir = std::env::temp_dir().join("utoipauto_exclude_alternates");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();

        fs::write(dir.join("controller.rs"), "pub fn handler() {}\n").unwrap();
        fs::write(dir.join("schema_test.rs"), "pub struct S;\n").unwrap();
        fs::write(dir.join("schema_gen.rs"), "pub struct G;\n").unwrap();

        let parsed = parse_files(&dir, &build_exclude_set(&["**/*_{test,gen}.rs"])).unwrap();
        assert_eq!(parsed.len(), 1);

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn test_extract_module_name_from_path() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/controller1.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_path_windows() {
        assert_eq!(
            extract_module_name_from_path(".\\utoipa-auto-macro\\tests\\controllers\\controller1.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::controllers::controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_mod() {
        assert_eq!(
            extract_module_name_from_path("./utoipa-auto-macro/tests/controllers/mod.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_lib() {
        assert_eq!(
            extract_module_name_from_path("./src/lib.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate"
        );
    }

    #[test]
    fn test_extract_module_name_from_main() {
        assert_eq!(
            extract_module_name_from_path("./src/main.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace() {
        assert_eq!(
            extract_module_name_from_path("./server/src/routes/asset.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_nested() {
        assert_eq!(
            extract_module_name_from_path("./crates/server/src/routes/asset.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders() {
        assert_eq!(
            extract_module_name_from_path("./src/routing/api/audio.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::routing::api::audio"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders_nested() {
        assert_eq!(
            extract_module_name_from_path("./src/applications/src/retail_api/controllers/mod.rs", "crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::retail_api::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_folders_nested_external_crate() {
        assert_eq!(
            extract_module_name_from_path("./src/applications/src/retail_api/controllers/mod.rs", "other_crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "other_crate::retail_api::controllers"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_with_prefix_path() {
        assert_eq!(
            extract_module_name_from_path("./crates/server/src/routes_lib/routes/asset.rs", "crate::routes")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "crate::routes::asset"
        );
    }

    #[test]
    fn test_extract_module_name_from_workspace_with_external_crate_and_underscore() {
        assert_eq!(
            extract_module_name_from_path("./src/applications/src/retail-api/controllers/mod.rs", "other-crate")
                .to_token_stream()
                .to_string()
                .replace(" ", ""),
            "other_crate::retail_api::controllers"
        );
    }
}
