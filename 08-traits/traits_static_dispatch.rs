trait Duplicatable {
    fn dupl(&self) -> String;
}

impl Duplicatable for String {
    fn dupl(&self) -> String {
        format!("{0},{0}", *self)
    }
}

impl Duplicatable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self * 2)
    }
}

fn duplicate<T: Duplicatable> (x: T) {
    println!("{}", x.dupl());
}

fn main(){
    let a = 42;
    duplicate(a);

    let b = "hello world".to_string();
    duplicate(b);
}