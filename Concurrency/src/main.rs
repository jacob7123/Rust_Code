use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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
    
}
