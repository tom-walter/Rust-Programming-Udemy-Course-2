use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;


const NUM_THREADS: usize = 12;

fn start_thread(d: usize, tx: Sender<usize>) {
    thread::spawn(move || {
        println!("setting time for {}s", d);
        thread::sleep(Duration::from_secs(d as u64));
        println!("sending message {}", d);
        tx.send(d).unwrap()
    });
}

fn main() {
    // send data from sub-thread to main-thread
    // let (tx, rx) = mpsc::channel();
    // thread::spawn(move || {tx.send(42).unwrap()});
    // println!("received {}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_THREADS {
        start_thread(i, tx.clone());
    }

    for j in rx.iter().take(NUM_THREADS) {
        println!("received {}", j);
    }
}
