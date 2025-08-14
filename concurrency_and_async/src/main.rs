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

<<<<<<< HEAD
//   await doesn't trigger execution - the Future starts executing as soon as you call the async function. What await does is:
// Check if the Future is ready (completed)
// If ready: return the result immediately
// If not ready: yield control back to the executor and suspend the current task

    trpl::run(async { // main task, it spawn anoher task via spawn_task
        let handle = trpl::spawn_task(async { // spwan hands it to the executor to start running immediately
            for i in 1..10 {
                println!(" hi number {} from the first task", i);
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });
        // here there is no more main task, so it COMPLETES immediately
        // unless we await here
       match handle.await { // handle.await creates a dependency relationship, but the executor still treats them as peer tasks
            Ok(()) => println!("Task completed successfully"),
            Err(e) => println!("Task failed: {:?}", e),
        }
    });

    trpl::run ( async {
        // message passing
        let (tx, mut rx) = trpl::channel();
        // let vals = vec![String::from("1"), String::from("2"), String::from("3"), String::from("4")];
        // for val in vals {
        //     tx.send(val).unwrap();
        //     trpl::sleep(Duration::from_millis(300)).await;
        // }
        // while let Some(value) = rx.recv().await {
        //     println!("received '{value}'");
        // }
        // print!("Got {} received value", received);

        let tx_fut = async {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    })

    // mutex and atomic reference counter arc
    // use mutex so only one thread can access and operate on data
    // use Arc for multiple ownership in a multi-thread context
    use std::sync::{Mutex, Arc};
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move ||{
            let mut mut_ref = counter.lock().unwrap();
            *mut_ref += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

    // concurrency in rust is implemented by std, not the language itself
    // Send: Safe to transfer ownership between threads
    // Sync: Safe to share references between threads (multiple threads can access simultaneously)
    // this is the end for chapter 16

    // chapter 17: futures, async and await
    // key points:
    // async will wrap the block/fn as a future. async block is lazy
    // await will do two things: Starts polling the future, Suspends if the future isn't ready yet

    use trpl::Html;
    async fn page_title (url: &str) -> Option<String> {
        let response_text = trpl::get(url).await.text().await;
        Html::parse(&response_text)
            .select_first("title")
            .map(|title_element| title_element.inner_html())
    }

    let args: Vec<String> = std::env::args().collect();
    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("title {}", title),
            None => println!("{url} had no title"),
        }
    })
}
