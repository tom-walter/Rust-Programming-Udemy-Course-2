#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person
}

fn main() {
    let p = Person{name: String::from("John")};
    let d = Dog{name: String::from("Rex"), owner: &p};

    println!("{:?}", d)
}