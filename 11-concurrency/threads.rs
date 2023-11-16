use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

fn main() {
    let mut thread_list = vec![];

    for i in 0..10 {
        let th = thread::spawn(move ||{
            sleep(Duration::from_millis(1000));
            println!("new thread {}", i)
        });
        thread_list.push(th)
    }


    for t in thread_list {
        t.join();
    }

    println!("main thread")
}