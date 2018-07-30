use std::path::PathBuf;
use std::fs;
use std::fs::DirEntry;

pub fn list_files_in_dir_recursively(dir: &PathBuf) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = Vec::new();
    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        let f: DirEntry = path.unwrap();
        let is_file = f.file_type().unwrap().is_file();
        if !is_file {
            v.extend(list_files_in_dir_recursively(&f.path()));
        } else {
            v.push(f.path());
        }
    }
    v
}