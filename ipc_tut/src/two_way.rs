use std::sync::mpsc;
use std::thread;

pub fn two_way_comm() {
    //creating  channels

    let (tx_to_worker, rx_in_worker) = mpsc::channel();
    let (tx_to_parent, rx_in_parent) = mpsc::channel();

    let handle = thread::spawn(move || {
        for msg in rx_in_worker {
            println!("Worker received: {:?}", msg);
            let response = format!("done with {}", msg);
            tx_to_parent.send(response).unwrap();
        }
    });

    let messages = ["A", "B", "C"];

    for msg in messages {
        tx_to_worker.send(msg).unwrap();

        let reply = rx_in_parent.recv().unwrap();
        println!("Parent got reply : {}", reply);
    }

    drop(tx_to_worker);
    handle.join().unwrap();
}
