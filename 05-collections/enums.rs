use crate::Person::Name;

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32),
  }

fn main(){
    let my_fav_color = Colors::Red;
    println!("{:?}", my_fav_color);

    let person = Name(String::from("John"));
    println!("{:?}", person);
}