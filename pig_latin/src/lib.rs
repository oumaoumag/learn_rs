pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Check if the word starts with a vowel
    let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
    if "aeiou".contains(first_char) {
        return format!("{}ay", text);
    }

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if is_vowel(chars[i]) {
            break;
        }
        // Special case: if 'q' is followed by 'u' and not at the start, skip 'u'
        if i > 0 && chars[i].to_lowercase().next().unwrap() == 'q' && 
           i + 1 < chars.len() && chars[i + 1].to_lowercase().next().unwrap() == 'u' {
            i += 2;
        } else {
            i += 1;
        }
    }

    if i >= chars.len() {
        return format!("{}ay", text);
    }

    let consonants: String = text.chars().take(i).collect();
    let rest: String = text.chars().skip(i).collect();
    format!("{}{}ay", rest, consonants)
}

fn is_vowel(c: char) -> bool {
    let c_lower = c.to_lowercase().next().unwrap();
    "aeiou".contains(c_lower)
}