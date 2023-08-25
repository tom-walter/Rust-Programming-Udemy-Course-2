
#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T,
}

#[derive(Debug)]
enum Colors<T> {
    Red(T),
    Green(T),
    Blue(T),
}

fn main(){
    let p1: Point<i32> = Point{x: 6, y: 8};
    let p2: Point<f32> = Point{x: 3.25, y: 4.63};

    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);

    let c1 = Colors::Red("#f00");
    let c2 = Colors::Red(255);

    println!("c1 = {:?}", c1);
    println!("c2 = {:?}", c2);
}