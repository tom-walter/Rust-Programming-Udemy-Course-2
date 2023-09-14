static mut R: i32 = 0;

fn main(){
    // a scope
    {
        let a = 3;
        println!("{}", a);
    }
    // println!("{}", a); // will error since a is out of scope

    // unsafe block
    unsafe {
        println!("R = {}", R);
    }

    // closure aka lambda
    let sum = |a: i32, b: i32| -> i32 {a + b};
    let res = sum(2, 3);
    println!("{}", res);

    // generic closure
    let gen = |x| {println!("received {}", x)};
    gen("hello");
    //gen(3) // error for type
    
}