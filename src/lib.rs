use std::fs::{read_dir,  DirEntry};
use std::path::Path;
use regex;
use error::WalkrError;

pub mod error;

/// Recursively search a directory for a files matching a regex recieve the corresponding &DirEntry as an argument to the provided closure.
///
/// # Arguments
///
/// * `dir` - [&Path](std::path::Path) of the directory to begin searching in
/// * `search_str` - [&String](std::string::String) containing the regex to match files on
/// * `cb` - `&Fn(&DirEntry)` to be called for each file match
/// # Example
///
/// ```
/// use std::env;
/// use std::path::Path;
/// use std::fs::{File};
/// use std::io::Read;
/// 
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
pub fn find(dir: &Path, search_str: &String, cb: &Fn(&DirEntry)) -> Result<(), WalkrError> {
    if dir.is_dir() {
        match read_dir(dir) {
            Ok(rd) => {
                for entry in rd {
                    match entry {
                        Ok(dir_entry) => {
                            let path = dir_entry.path();
                            if path.is_dir() {
                                find(&path, search_str, cb)?;
                            } else {
                                match dir_entry.file_name().into_string() {
                                    Ok(file_name) => {
                                        match regex::Regex::new(format!(r#"{}"#, search_str).as_str()) {
                                            Ok(re) => {
                                                if re.is_match(file_name.as_str()) {
                                                    cb(&dir_entry);
                                                }
                                            },
                                            Err(e) => {
                                                return Err(WalkrError::new(error::Error::RegexError(e)));
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        println!("Couldn't convert filename {:?} into a string", e);
                                    }
                                };
                            }
                        },
                        Err(e) => {
                            return Err(WalkrError::new(error::Error::IOError(e)));
                        }
                    };
                }
            },
            Err(e) => {
                return Err(WalkrError::new(error::Error::IOError(e)));
            }
        };
    }
    Ok(())
}
