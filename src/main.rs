#![allow(dead_code)]

use std::env;
use std::path::Path;
use std::fs::{File};
use std::io::Read;

mod findr;

fn main() {
    let args: Vec<String> = env::args().collect();
    match findr::visit_dirs(Path::new(&args[1]), &args[2], &|d| {
        let mut f = File::open(d.path()).unwrap();
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                println!("{:?}", s);
            },
            _ => (),
        }
        println!("{:?} matched", d.file_name().into_string());
    }) {
      Ok(_) => println!("done"),
      Err(e) => panic!(e)
    }
}
