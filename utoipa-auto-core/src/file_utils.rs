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

pub fn parse_files<T: Into<PathBuf>>(path: T) -> Result<Vec<(String, syn::File)>, io::Error> {
    let mut files: Vec<(String, syn::File)> = vec![];

    let pb: PathBuf = path.into();
    if pb.is_file() {
        files.push((pb.to_str().unwrap().to_string(), parse_file(pb)?));
    } else {
        for entry in fs::read_dir(pb)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push((path.to_str().unwrap().to_string(), parse_file(path)?));
            }
        }
    }
    Ok(files)
}

pub fn extract_module_name_from_path(path: &String) -> String {
    let mut path = path.to_string();
    if path.ends_with(".rs") {
        path = path.replace(".rs", "");
    }
    if path.ends_with("/mod") {
        path = path.replace("/mod", "");
    }
    path = path.split('/').last().unwrap().to_string();
    path.replace('/', "::")
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
            "controller1"
        );
    }

    #[test]
    fn test_extract_module_name_from_mod() {
        assert_eq!(
            extract_module_name_from_path(
                &"./utoipa-auto-macro/tests/controllers/mod.rs".to_string()
            ),
            "controllers"
        );
    }
}
