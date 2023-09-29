struct Dog {}
struct Cat {}

trait Animal {
    fn make_noise(&self) -> &'static str;
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str{
        "woof"
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str{
        "meow"
    }
}

fn get_animal(number: f64) -> Box<dyn Animal> {
    if number < 1.0 {
        Box::new(Dog{})
    } else {
        Box::new(Cat{})
    }
}

fn main() {
    println!("The animal says {}", get_animal(0.0).make_noise());
    println!("The animal says {}", get_animal(2.0).make_noise());
}