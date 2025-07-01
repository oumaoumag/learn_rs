pub fn capitalize_first(input: &str) -> String {
    // Handle empty string case
    if input.is_empty() {
        return String::new();
    }
    
    // Get the first character and capitalize it
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => {
            let first_upper = first_char.to_uppercase().collect::<String>();
            // Combine the capitalized first character with the rest of the string
            first_upper + chars.as_str()
        }
    }
}

pub fn title_case(input: &str) -> String {
    // If the input is empty, return an empty string
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().collect::<String>());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
}

pub fn change_case(input: &str) -> String {
    // For each character, swap its case
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else {
                // Non-alphabetic characters remain unchanged
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing");
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A");
        assert_eq!(title_case(""), "");
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
        assert_eq!(change_case("123!@#"), "123!@#");
    }
}
