// lib.rs - Main implementation for ref_cell

// Import the messenger module
pub mod messenger;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use messenger::Logger;

// Worker structure that implements the Logger trait
pub struct Worker {
    pub track_value: Rc<i32>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    // Initialize a new Worker
    pub fn new(value: i32) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

// Implement the Logger trait for Worker
impl Logger for Worker {
    fn warning(&self, msg: &str) {
        let message = format!("Warning: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(message);
    }

    fn info(&self, msg: &str) {
        let message = format!("Info: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(message);
    }

    fn error(&self, msg: &str) {
        let message = format!("Error: {}", msg);
        self.mapped_messages
            .borrow_mut()
            .insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(message);
    }
}
