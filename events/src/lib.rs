use chrono::Duration;
use colored::*;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.color;
        let colored_content = self.content.truecolor(r, g, b);
        write!(f, "({:?}, {}, {})", self.position, self.size, colored_content)
    }
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
       match self {
            Event::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },

            Event::Registration(duration) => {
               let hours =  duration.num_hours();
                let minutes = duration.num_minutes() % 60;
                let seconds = duration.num_seconds() % 60;
            
            Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!("You have {}H:{}M: {}5 left before the registration ends", hours, minutes, seconds)
               }
            },

            Event::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),               
            },

            Event::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enyoy your holiday".to_string(),
            },
        } 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications() {
        let remainder = Event::Remainder("Test");
        let notification = remainder.notify();
        assert_eq!(notification.size, 50);
    }
}
