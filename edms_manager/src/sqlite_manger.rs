use std::collections::{HashSet, VecDeque};
use crate::domain::Endpoint;
// use std::sync::{Arc, Mutex};

pub struct SqliteLockManager {
    sqlitefilelist_readonly: HashSet<String>,
    sqlitefilelist_write: VecDeque<String>,
}

impl SqliteLockManager {
    pub fn new() -> Self {
        Self {
            sqlitefilelist_readonly: HashSet::new(),
            sqlitefilelist_write: VecDeque::new(),
        }
    }

    pub fn add_reader(&mut self, file_path: String) {
        self.sqlitefilelist_readonly.insert(file_path);
    }

    pub fn remove_reader(&mut self, file_path: &str) {
        self.sqlitefilelist_readonly.remove(file_path);
    }

    pub fn add_writer(&mut self, file_path: String) {
        self.sqlitefilelist_write.push_back(file_path);
    }

    //--- Write OPERATIONS (Deque/FIFO)---

    pub fn process_next_writer(&mut self) -> Option<String> {
        self.sqlitefilelist_write.pop_front()
    }

    pub fn writer_queue_count(&self) -> usize {
        self.sqlitefilelist_write.len()
    }
}

pub fn mock_fetch_from_sqlite() -> Vec<Endpoint> {
    // streaming data from sqlite
    //we are not using fetch all, as it will eat up a lot of RAM.
    //using rusqlite
    (1..=2500)
        .map(|i| Endpoint {
            eid: i,
            eid_str: format!("EID-{}", i),
            ep_type: "GET".to_string(),
            annotation: "Auto-generated".to_string(),
            tags: "V1, test".to_string(),
            req_res_count: 10,
        })
        .collect()
}

