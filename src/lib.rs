use std::fs::{read_dir,  DirEntry};
use std::io;
use std::path::Path;
use regex::Regex;

pub fn find(dir: &Path, search_str: &String, cb: &Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find(&path, search_str, cb)?;
            } else {
                match entry.file_name().into_string() {
                    Ok(file_name) => {
                        match Regex::new(format!(r#"{}"#, search_str).as_str()) {
                            Ok(re) => {
                                if re.is_match(file_name.as_str()) {
                                    cb(&entry);
                                }
                            },
                            Err(e) => {
                                panic!(e);
                            }
                        }
                    },
                    Err(e) => {
                        panic!(e);
                    }
                }
            }
        }
    }
    Ok(())
}
