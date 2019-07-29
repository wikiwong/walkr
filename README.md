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

_Running this example:_
```
cargo run ./src \.rs
```
