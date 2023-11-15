use std::fs::{File, OpenOptions, remove_file};
use std::io::{Write, Read};

fn main() {
    // file creation
    let mut file = File::create("./example.txt").expect("failed");
    
    // writing to a file 
    // file.write_all("hello world\n".as_bytes()).expect("write failed");

    // appending a file 
    let mut file = OpenOptions::new()
        .append(true)
        .open("./example.txt")
        .expect("failed");

    file.write_all("hello world\n"
        .as_bytes())
        .expect("append failed");

    file.write_all("another line of text\n"
        .as_bytes())
        .expect("append failed");

    // reading from a file
    let mut file = File::open("./example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);

    // file delection
    remove_file("./example.txt").expect("delete failed");
}