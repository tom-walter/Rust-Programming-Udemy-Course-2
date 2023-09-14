fn apply(f: fn(i32)->i32, a:i32){
    println!("results {}", f(a));
}

fn is_even(number: i32) -> bool{
    number % 2 == 0
}

fn main(){

    // higher order function
    let square = |a: i32| a * a;
    apply(square, 6);

    // method chaining, higher order functions
    let limit = 500;
    let sum_sqrd = (0..)
        // map applies a function to values of a range
        .map(|x| x*x)
        // take_while takes only values from range until given condition
        .take_while(|&x| limit>x) 
        // filter filters given a certain condition
        .filter(|x| is_even(*x))
        // fold aggregates range values into single value by given func
        .fold(0, |sum, x| sum + x); 

    println!("sum sqrd = {}", sum_sqrd);

}