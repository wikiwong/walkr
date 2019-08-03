_WARNING: This crate will evolve over time to provide conveniences when interacting with the file system, however in its current state, it is mainly being used to help me get familiar with publishing crates_

# walkr

`walkr` recursively searches a directory for files matching a regex, and executes a closure you define, taking in a [&DirEntry](https://doc.rust-lang.org/std/fs/struct.DirEntry.html).  This tiny crate allows you to find and operate on files quickly, making it convenient to quickly write tools performing file processing in Rust.

## Usage
```rust
match walkr::find(Path::new("./"), &"\\.rs".to_owned(), &|d| {
  println!("File: {:?} matched!", d.file_name().into_string().unwrap());
  
  // open the file and print the contents to stdout
  let mut f = File::open(d.path()).unwrap();
  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => {
      println!("{:?}", s);
    },
    Err(e) => panic!(e)
  }
}) {
  Ok(_) => println!("done"),
  Err(e) => panic!(e)
}
```

_Running the example in `./src/main.rs`:_
```
# in the root of this repo
cargo run ./src \.rs
```
