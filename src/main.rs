#![allow(dead_code)] 

use std::path::Path;
use std::env;
use std::io;
use std::fs::{File, read_dir, DirEntry};
use std::io::prelude::*;

fn connect(ip: (u8, u8, u8, u8)) {
  match ip {
    (hi) => { println!("I'm the last octet! {:?}", hi); },
  }
}

fn cat(p: &Path) -> io::Result<String> {
  let mut f = File::open(p)?;
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e)
  }
}

fn file_match(d: &DirEntry, search_str: &String) {
  if d.file_name().into_string().unwrap().contains(search_str) {
    println!("{:?}", d.path());
  }
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, search_str: &String) -> io::Result<()> {
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

fn main() {
    connect((192, 168, 1));
    // let args: Vec<String> = env::args().collect();
    // match visit_dirs(Path::new(&args[1]), &args[2]) {
    //   Ok(_) => println!("done"),
    //   Err(e) => panic!(e)
    // }


    // match cat(Path::new("Cargo.locky")) {
    //   Ok(s) => println!("{}", s),
    //   Err(e) => panic!("{}", e)
    // }
}
