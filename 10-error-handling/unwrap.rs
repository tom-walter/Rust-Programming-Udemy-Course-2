use std::fs::File;

fn main() {
    let file = File::open("meme.jpg").unwrap();
    /* unwwap is equivalent to following match
    match f {
        Ok(f) => {
            println("file found {:?}", f);
        }
        Err(f) => {
            panic!()
        }
    }
    */
}

