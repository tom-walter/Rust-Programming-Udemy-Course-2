const DEFAULT: i32 = 3;

fn main(){
    let primes = [2, 3, 5, 7, 11];
    let doubles: [f64; 4] = [2.0, 4.0, 6.0, 8.0];

    println!("{:?}", primes);
    println!("{:?}", doubles);

    let zeros = [0; 10];
    let mut threes = [DEFAULT; 5];

    println!("{:?}", zeros);
    println!("{:?}", threes);

    threes[3] = 5;
    println!("{:?}", threes);
}