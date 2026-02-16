use std::sync::mpsc;
use std::thread;

pub fn multi_worker_method() {
    //creating a channel

    let (tx, rx) = mpsc::channel();
    //rx for replication
    let rx = std::sync::Arc::new(std::sync::Mutex::new(rx));

    for id in 0..3 {
        let rx_clone = rx.clone();

        thread::spawn(move || {
            loop {
                let msg = rx_clone.lock().unwrap().recv();
                match msg {
                    Ok(m) => println!("Worker {} got {}", id, m),
                    Err(_) => break,
                }
            }
        });
    }

    for i in 0..10 {
        tx.send(i).unwrap();
    }
    drop(tx);
    thread::sleep(std::time::Duration::from_secs(1));
}
