use std::{thread, time::Duration, vec};
use std::sync::mpsc;

fn main() {
    // thread::spawn creates sub thread, which ends when main thread ends
    thread::spawn(||{
        for i in 0..10 {
            println!(" this is a sub1 thread, counting {}", i);
            thread::sleep(Duration::from_millis(300));
        }
        
    });

    for i in 0..2 {
        println!(" this is main thread, counting {}", i);
        thread::sleep(Duration::from_millis(300));
    } 

    // use join on over a handle so that it will wait for its thread to finish before main exits 
    let handle = thread::spawn(||{
        for i in 0..5 {
            println!(" this is a sub2 thread, counting {}", i);
            thread::sleep(Duration::from_millis(300));
        }
        
    }); 

    // careful, this is a blocking statement
    handle.join().unwrap();

    println!("finishing the main");

    // closure with move
    let v = vec![1, 2, 3];
    thread::spawn(move || {
        println!("use move for closure, printing v = {:?}", v); 
    });

    // use channel communicate
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("this is a test").unwrap(); // unwrap will return error when the receiver has already been dropped and there’s nowhere to send
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    let (tx, rx) = mpsc::channel();
    // use clone to create multiple producer
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx1:hi"),
            String::from("tx1:from"),
            String::from("tx1:the"),
            String::from("tx1:thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    
    
}
