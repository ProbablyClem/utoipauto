use std::{fs::File, io::Read, path::PathBuf};

pub fn parse_file<T: Into<PathBuf>>(filepath: T) -> Result<syn::File, ()> {
    let pb: PathBuf = filepath.into();

    if pb.is_file() {
        let mut file = File::open(pb).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let sf = syn::parse_file(&content).unwrap();
        Ok(sf)
    } else {
        Err(())
    }
}
