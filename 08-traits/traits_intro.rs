struct RustDev {
    awesome: bool
}

struct JavaDev {
    awesome: bool
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) { 
        println!("Hello world!")
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Rust"
    }

    fn say_hello(&self) {
        println!("println!(\"Hello worlds!\");");
    }
}

impl Developer for JavaDev {
    fn new(awesome: bool) -> Self {
        JavaDev{awesome: awesome}
    }

    fn language(&self) -> &str {
        "Java"
    }

    fn say_hello(&self) {
        println!("System.out.println(\"Hello worlds!\");");
    }
}


fn main(){
    let r = RustDev::new(true);
    let j = JavaDev::new(false);

    println!("{}", r.language());
    r.say_hello();

    println!("{}", j.language());
    j.say_hello();
}