#![allow(dead_code)]

use std::env;
use std::path::Path;

mod findr;

fn main() {
    let args: Vec<String> = env::args().collect();
    match findr::visit_dirs(Path::new(&args[1]), &args[2]) {
      Ok(_) => println!("done"),
      Err(e) => panic!(e)
    }
}
