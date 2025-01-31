// use std::rc::Rc;
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
use std::sync::{Arc, Mutex};

fn main() {
    /*
       Spawn and Join
    */

    /*  Need to sleep for waiting thread working. */
    // std::thread::spawn(move ||{
    //     println!("Hello from thread!");
    // });

    // thread::sleep(Duration::from_secs(1));

    /* Using join let thread working before out of scope. */
    // let handle = std::thread::spawn(move ||{
    //         println!("Hello from thread!");
    // });
    // handle.join().unwrap();

    // println!("Hello from main!");

    // let v = vec![1, 2, 3];
    // let handle = std::thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    // let mut thread_handle = Vec::new();
    // for i in v{
    //     thread_handle.push(std::thread::spawn(move || println!("Thread {}", i)));
    // }

    // println!("Main Thread!");

    // for handle in thread_handle{
    //     handle.join().unwrap();
    // }

    // println!("Main Thread!");

    /*
       Channels
    */

    // let (transmitter, receiver) = mpsc::channel();
    // let tx = transmitter.clone();

    // let val = String::from("Transmitting!!!!!!!!!!!!");
    // std::thread::spawn(move ||{
    //     transmitter.send(val).unwrap();
    // });
    // let msg = receiver.recv().unwrap();
    // println!("{}", msg); // val ownership moves to msg.

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Transmitting"),
    //         String::from("From"),
    //         String::from("Original"),
    //     ];
    //     for val in vec {
    //         transmitter.send(val).unwrap();
    //     }
    // });

    // std::thread::spawn(move || {
    //     let vec = vec![
    //         String::from("Clone"),
    //         String::from("Is"),
    //         String::from("Transmitting"),
    //     ];
    //     for val in vec {
    //         tx.send(val).unwrap();
    //     }
    // });

    // for rec in receiver {
    //     println!("{}", rec);
    // }

    /*
         Send and Sync
    */

    /* Rc is not thread safe, so it cannot send to thread. */
    // let rc1 = Rc::new(String::from("Test")); // Use Arc::new(String::from("Test")
    // let rc2 = rc1.clone();

    // std::thread::spawn(move || {
    //     rc2;
    // });

    /*
        Shared State
    */

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}
