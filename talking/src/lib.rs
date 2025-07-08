pub fn talking(text: &str) -> &str {
    let trimmed_text = text.trim();

    let is_empty = trimmed_text.is_empty();

    let is_question = trimmed_text.ends_with("?");

    let is_yelling = {
        let mut has_alpha = false; 
        let mut all_alpha_uppercase = true; 
        for c in trimmed_text.chars() {
            if c.is_alphabetic() {
                has_alpha = true; // Found an alphabetic character
                if !c.is_ascii_uppercase() {
                    all_alpha_uppercase = false;
                    break; // No need to check further
                }
            }
        }
        has_alpha && all_alpha_uppercase
    };

    if is_empty {
        "Just say something!"
    } else if is_yelling && is_question {
        "Quiet, I am thinking!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}