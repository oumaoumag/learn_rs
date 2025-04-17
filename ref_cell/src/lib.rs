pub mod messenger;

use std::{collections::HashMap, cell::RefCell, rc::Rc};

pub use messenger::{Logger, Tracker};

pub struct Worker<T> {
    pub track_value: Rc<T>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl<T> Worker<T> {
    pub fn new(value: T) -> Self {
        Worker {
            track_value: Rc::new(value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl<T> Logger for Worker<T> {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Warning".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }

    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Info".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }

    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert("Error".to_string(), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}
