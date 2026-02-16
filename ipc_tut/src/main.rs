use std::sync::mpsc;
use std::thread;

fn main() {
    println!("IPC tutorial");

    //create a channel
    let (tx, rx) = mpsc::channel();

    //spawn a worker process
    let handle = thread::spawn(move || {
        //receive a message
        let received_msg = rx.recv();
        println!("Worker received {:?}", received_msg);

        //simulate work done
        println!("Worker -> Done");
    });

    //parent to send the data
    tx.send("hello").unwrap();
    println!("Parent -> hello");

    //wait till all tasks are finished
    handle.join().unwrap();
}
