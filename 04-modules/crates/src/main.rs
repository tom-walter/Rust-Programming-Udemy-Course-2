extern crate rand;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen();
    println!("random number {}", num);  
}
