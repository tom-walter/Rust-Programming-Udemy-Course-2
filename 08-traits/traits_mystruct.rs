trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut sum: i32 = 0;
        for n in self {
            sum += *n;
        }
        return sum
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    println!("sum of vec {:?} = {}", a, a.sum());

}