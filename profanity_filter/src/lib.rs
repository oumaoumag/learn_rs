pub struct Message {
    content: String,
    user: String,
}

impl Message {
    pub fn new(content: String, user: String) -> Self {
        Self { content, user }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.to_lowercase().contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    let msg = Message::new(message.to_string(), String::new());
    match msg.send_ms() {
        Some(content) => Ok(message),
        None => Err("ERROR: illegal"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_message() {
        assert_eq!(check_ms("hello there"), Ok("hello there"));
    }

    #[test]
    fn test_empty_message() {
        assert_eq!(check_ms(""), Err("ERROR: illegal"));
    }

    #[test]
    fn test_profanity() {
        assert_eq!(check_ms("you are stupid"), Err("ERROR: illegal"));
        assert_eq!(check_ms("stupid"), Err("ERROR: illegal"));
        assert_eq!(check_ms("STUPID message"), Err("ERROR: illegal"));
    }
}
