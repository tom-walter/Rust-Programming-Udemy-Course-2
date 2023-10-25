
fn main(){
    // transferring ownership of primitives
    // this copies the value! 
    let i: i32 = 5;
    let j = i;
    // proof: both variables can be used
    println!("i={}", i);
    println!("j={}", j);

    // transferring ownership of structs
    // this transfers access of data in memory
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let w = v;
    println!("w={:?}", w);
    //println!("v={:?}", v); // will error out

    // lambda expressions
    let foo = |v: Vec<i32>| -> Vec<i32>
    {
        println!("vector used in foo:");
        return v
    };
    let w = foo(w);
    println!("w={:?}", w);

}