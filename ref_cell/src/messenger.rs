use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a, T: Logger> {
    pub logger: &'a T,
    pub value: usize,
    pub max: usize,
}

impl<'a, T: Logger> Tracker<'a, T> {
    pub fn new(logger: &'a T, max: usize) -> Self {
        Tracker {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value<U>(&self, rc_value: &Rc<U>) {
        let count = Rc::strong_count(rc_value);
        let percentage = (count as f64 / self.max as f64) * 100.0;

        if percentage >= 100.0 {
            self.logger.error("Error: you are over your quota!");
        } else if percentage >= 70.0 {
            let message = format!("Warning: you have used up over {}% of your quota! Proceeds with precaution", percentage.round() as usize);
            self.logger.warning(&message);
        }
    }

    pub fn peek<U>(&self, rc_value: &Rc<U>) {
        let count = Rc::strong_count(rc_value);
        let percentage = (count as f64 / self.max as f64) * 100.0;
        let message = format!("Info: you are using up to {}% of your quota", percentage.round() as usize);
        self.logger.info(&message);
    }
}