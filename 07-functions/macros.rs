macro_rules! my_macro {
    () => {println!("First Macro!")}
}

macro_rules! name {
    ($name: expr) => {println!("Hey {}!", $name)} 
} 

macro_rules! numbers {
    ($($num: expr), *) => ( $(println!("Number = {}", $num);) * )
}

macro_rules! xy {
    (x => $x: expr) => (println!("X is {}", $x));
    (y => $y: expr) => (println!("Y is {}", $y));
} 

macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name(){}
            println!("{:?} was called", stringify!($fn_name));
    }
}

fn main(){
    my_macro!();

    name!("John Doe");

    numbers!("1", "2", "3", "4", "5");

    xy!(x => 5);
    xy!(y => 5 * 9);

    build_fn!(hello);
    hello();
}