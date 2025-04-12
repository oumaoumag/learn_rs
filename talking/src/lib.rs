pub fn talking(message: &str) -> String {
    if message.trim().is_empty() {
        return String::from("Just say something!");
    }

    let has_letters = message.chars().any(|c| c.is_alphabetic());
    
    let is_yelling = has_letters && message.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());
    
    let is_question = message.trim().ends_with("?");
    
    match (is_yelling, is_question) {
        (true, true) => String::from("Quiet, I am thinking!"),
        (true, false) => String::from("There is no need to yell, calm down!"),
        (false, true) => String::from("Sure."),
        (false, false) => String::from("Interesting"),
    }
}