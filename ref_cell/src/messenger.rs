// messenger.rs - Module for tracking and limiting references
use std::cell::Cell;

// Logger trait for displaying messages
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// Tracker structure to monitor reference counts
pub struct Tracker<'a> {
    logger: &'a dyn Logger,
    pub value: Cell<usize>,
    max: usize,
}

impl<'a> Tracker<'a> {
    // Initialize a new Tracker
    pub fn new(logger: &'a dyn Logger, max: usize) -> Self {
        Tracker {
            logger,
            value: Cell::new(0),
            max,
        }
    }

    // Set the value and check if it exceeds thresholds
    pub fn set_value(&self, value: &std::rc::Rc<i32>) {
        // Use Cell to update the value field
        let count = std::rc::Rc::strong_count(value);
        self.value.set(count);

        let percentage = (self.value.get() as f32 / self.max as f32) * 100.0;

        if percentage >= 100.0 {
            self.logger.error("you are over your quota!");
        } else if percentage >= 70.0 {
            // Round to the nearest 10 for the 80% case
            let rounded_percentage = if percentage >= 80.0 && percentage < 90.0 {
                80
            } else {
                percentage.round() as i32
            };

            self.logger.warning(&format!(
                "you have used up over {}% of your quota! Proceeds with precaution",
                rounded_percentage
            ));
        }
    }

    // Peek at the current usage percentage
    pub fn peek(&self, value: &std::rc::Rc<i32>) {
        // Use Cell to update the value field
        let count = std::rc::Rc::strong_count(value);
        self.value.set(count);

        let percentage = (self.value.get() as f32 / self.max as f32) * 100.0;

        self.logger.info(&format!(
            "you are using up to {}% of your quota",
            percentage.round() as i32
        ));
    }
}
