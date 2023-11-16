use std::sync::{Mutex, Arc};
use std::thread;

const NUM_THREADS: usize = 12;

fn main() {
    let c = Arc::new(Mutex::new(0));

    let mut thread_list = vec![];

    for _ in 0..NUM_THREADS {
        let c = Arc::clone(&c);
        let t = thread::spawn(move || {
            let mut num = c.lock().unwrap();
            *num += 1; 
        });
        thread_list.push(t);
    }

    for th in thread_list {
        th.join().unwrap();
    }

    println!("result {}", *c.lock().unwrap());
}