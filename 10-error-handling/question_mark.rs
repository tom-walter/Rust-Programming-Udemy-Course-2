use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

#[allow(unused_variables)]
#[allow(unused_assignments)]


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s)
}

// this is equivalent to the ? operator
// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("username.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

fn main() {
    let user = read_username_from_file();
    println!("{:?}", user);
}