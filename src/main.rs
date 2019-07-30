use std::env;
use std::path::Path;
use std::fs::{File};
use std::io::Read;

use walkr;

// Find a file {{f}} in directory {{d}} and print it's contents:
// cargo run {{d}} {{f}}
fn main() {
    let args: Vec<String> = env::args().collect();
    match walkr::find(Path::new(&args[1]), &args[2], &|d| {
        println!("File: {:?} matched!", d.file_name().into_string().unwrap());
        let mut f = File::open(d.path()).unwrap();
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => {
                println!("{:?}", s);
            },
            _ => (),
        }
    }) {
      Ok(_) => println!("done"),
      Err(e) => {
          panic!("{:?}", e);
      }
    };
}
