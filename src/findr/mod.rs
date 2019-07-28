use std::fs::{read_dir,  DirEntry};
use std::io;
use std::path::Path;

pub fn visit_dirs(dir: &Path, search_str: &String, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, search_str, cb)?;
            } else if entry.file_name().into_string().unwrap().contains(search_str) {
                cb(&entry);
            }
        }
    }
    Ok(())
}
