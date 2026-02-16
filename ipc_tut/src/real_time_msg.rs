use std::sync::mpsc;
use std::thread;

enum Command {
    Insert(i32, String),
    Fetch(i32),
    Shutdown,
}

pub fn real_time_comm() {
    //create channel
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut db = Vec::new();

        for cmd in rx {
            match cmd {
                Command::Insert(id, msg) => {
                    db.push((id, msg));
                    println!("Inserted {}", id);
                }

                Command::Fetch(id) => {
                    println!("Fetch request {}", id);
                }

                Command::Shutdown => {
                    println!("Shutting down worker");
                    break;
                }
            }
        }
    });

    tx.send(Command::Insert(1, "hello".into())).unwrap();
    tx.send(Command::Fetch(1)).unwrap();
    tx.send(Command::Shutdown).unwrap();
}
