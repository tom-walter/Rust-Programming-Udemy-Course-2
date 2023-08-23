extern crate rand;
use rand::Rng;
use rand::distributions::Alphanumeric; 

fn main() {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen();
    println!("random number {}", num);  

    println!("bounded int: {}", rng.gen_range(0, 100));
    println!("bounded float: {}", rng.gen_range(0.0, 100.0));

    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .collect();
    println!("Gen string: {}", rand_string); 
}
