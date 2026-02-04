use std::{thread, time::Duration};
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut number = counter.lock().unwrap();
            *number += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result {}", *counter.lock().unwrap());
}

