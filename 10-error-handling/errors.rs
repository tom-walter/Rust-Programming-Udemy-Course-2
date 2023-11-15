use std::fs::File;
use std::io::Error;

fn main() {
    // unrecoverable error
    // let v: Vec<i32> = (0..5).collect();
    // v[10];

    let div = |a: f64, b: f64| -> f64 {
        if b == 0.0 {
            panic!("Zero Division Error")
        }
        else {
            return a / b
        }
    };

    // let div_result = div(5.0, 0.0);

    let file: Result<File, Error> = File::open("meme.jpg");
    match file {
        Ok(file) => {println!("File opened: {:?}", file)},
        Err(e) => {println!("File not found: {:?}", e)}
    }
    println!("Continuing program ...")

}