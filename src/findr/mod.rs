use std::fs::{read_dir, read_to_string, DirEntry, File};
use std::io;
use std::io::Read;
use std::path::Path;

pub fn visit_dirs(dir: &Path, search_str: &String) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, search_str)?;
            } else {
                file_match(&entry, search_str);
            }
        }
    }
    Ok(())
}

// example of something to do when a file is found
fn file_match(d: &DirEntry, search_str: &String) {
    if d.file_name().into_string().unwrap().contains(search_str) {
        let mut f = File::open(d.path()).unwrap();
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                println!("{:?}", s);
            },
            _ => (),
        }
        println!("{:?} matched", d.file_name().into_string());
    }
}
