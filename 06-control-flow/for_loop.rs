fn main(){
    for n in 1..=10 {
        print!("{}, ", n);
    }
    print!("\n");

    let pets = ["cat", "dog", "chihuahua", "hamster", "bird"];
    for pet in pets.iter() {
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue
        }
        println!("I love my {}", pet);
    }
}