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

pub fn parse_files<T: Into<PathBuf>>(path: T) -> Result<Vec<syn::File>, io::Error> {
    let mut files: Vec<syn::File> = vec![];

    let pb: PathBuf = path.into();
    if pb.is_file() {
        files.push(parse_file(pb)?);
    } else {
        for entry in fs::read_dir(pb)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(parse_file(path)?);
            }
        }
    }
    Ok(files)
}
