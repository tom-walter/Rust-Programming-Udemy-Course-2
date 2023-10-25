fn main() {
    let mut a: i32 = 6;
    let b = &mut a;

    println!("b={}", *b);
    // now b was consumed
    // borrowed ownership reverts
    println!("a={}", a);

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    for num in &v {
        println!("{}", num);
    }

}