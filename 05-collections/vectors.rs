const DEFAULT: bool = true;

fn main(){

    let mut primes = vec![2, 3, 5, 7];
    println!("primes {:?}", primes);
    primes.push(11);
    println!("primes {:?}", primes);
    primes.remove(0);
    println!("primes {:?}", primes);

    let vec1 = vec![2; 10];
    let vec2 = vec![DEFAULT; 5];
    println!("vec1 = {:?}", vec1);
    println!("vec2 = {:?}", vec2);

}