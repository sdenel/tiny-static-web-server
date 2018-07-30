use std::path::PathBuf;

pub fn does_gz_version_exists(filepath: &PathBuf) -> bool {
    let gz_filename = filepath.to_str().unwrap().to_owned() + ".gz";
    let gz_pathbuf = PathBuf::from(gz_filename);
    gz_pathbuf.exists()
}