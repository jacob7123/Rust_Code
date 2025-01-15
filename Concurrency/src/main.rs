use std::thread;
use std::time::Duration;
fn main() {
    /*
        Need to sleep for waiting thread working.
     */
    // std::thread::spawn(move ||{
    //     println!("Hello from thread!");
    // });

    // thread::sleep(Duration::from_secs(1));

    /*
        Using join let thread working before scope end.
     */
    // let handle = std::thread::spawn(move ||{
    //         println!("Hello from thread!");
    // });
    // handle.join().unwrap();

    // println!("Hello from main!");

    let v = vec![1, 2, 3];
    // let handle = std::thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    let mut thread_handle = Vec::new();
    for i in v{
        thread_handle.push(std::thread::spawn(move || println!("Thread {}", i)));
    }

    println!("Main Thread!");

    for handle in thread_handle{
        handle.join().unwrap();
    }

    println!("Main Thread!");


}
