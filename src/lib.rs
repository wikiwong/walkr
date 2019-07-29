use std::fs::{read_dir,  DirEntry};
use std::io;
use std::path::Path;
use regex::Regex;

/// Recursively search a directory for a files matching a regex and execute a closure on that regex.
///
/// # Arguments
///
/// * `dir` - [&Path](std::path::Path) of the directory to begin searching in
/// * `search_str` - [&String](std::string::String) containing the regex to match files on
/// * `cb` - `&Fn(&DirEntry)` to be called for each file match
/// # Example
///
/// ```
/// match walkr::find(Path::new("./"), &"\\.rs".to_owned(), &|d| {
///   println!("File: {:?} matched!", d.file_name().into_string().unwrap());
///
///   // open the file and print the contents to stdout
///   let mut f = File::open(d.path()).unwrap();
///   let mut s = String::new();
///   match f.read_to_string(&mut s) {
///     Ok(_) => {
///       println!("{:?}", s);
///     },
///     Err(e) => panic!(e)
///   }
/// }) {
///   Ok(_) => println!("done"),
///   Err(e) => panic!(e)
/// }
/// ```
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
