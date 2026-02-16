mod multi_worker;
mod real_time_msg;
mod two_way;
use multi_worker::multi_worker_method;
use real_time_msg::real_time_comm;
use std::sync::mpsc;
use std::thread;
use two_way::two_way_comm;

// level 1
fn main() {
    println!("IPC tutorial");

    //create a channel
    let (tx, rx) = mpsc::channel();

    let mut db = vec![(0, String::from("message0"))];
    //spawn a worker process
    let handle = thread::spawn(move || {
        //receive a message
        // let received_msg = rx.recv();
        // println!("Worker received {:?}", received_msg);
        for received_msg in rx {
            println!("Pushed message {:?} to local db", received_msg);
            db.push(received_msg); // vec::push() it takes ownership of value that we push
        }
        //simulate work done
        println!("Worker -> Done");
        print!("Final db: {:?}", db);
    });

    let messages = ["message1", "message2", "message3"];

    for (message_id, message) in messages.iter().enumerate() {
        tx.send((message_id, message.to_string())).unwrap();
    }
    //parent to send the data
    println!("Parent -> Sent messages from array to receiver");

    drop(tx); // very important because not dropping will keep worker process alive
    //and program will stay live
    //wait till all tasks are finished
    handle.join().unwrap();
    println!("\n======= Two Way Communication ========");
    two_way_comm();

    println!("\n ========= Multi Worker ==========");
    multi_worker_method();

    println!("\n ====== Real time communication =====");
    real_time_comm();
}
