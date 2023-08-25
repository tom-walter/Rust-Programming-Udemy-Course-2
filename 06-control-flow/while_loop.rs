fn get_squares(limit: i32){
    let mut x = 1;

    while x * x < limit {
        println!("{0} * {0} = {1}", x, x*x);
        x += 1;
    }
}

fn get_cubes(limit: i32){
    let mut x: i32 = 1;

    loop {
        if x.pow(3) > limit {break}
        println!("{0} ^ 3 = {1}", x, x.pow(3));
        x += 1; 
    }
}

fn main(){
    let limit = 3500;
    get_squares(limit);
    get_cubes(limit);
}