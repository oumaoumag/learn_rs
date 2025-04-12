pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Check if the word starts with a vowel
    let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
    
    if "aeiou".contains(first_char) {
        // If word begins with a vowel, add "ay" to the end
        return format!("{}ay", text);
    }

    // For words starting with consonants
    let chars: Vec<char> = text.chars().collect();
    
    // Find the position of the first vowel
    let mut vowel_pos = 0;
    while vowel_pos < chars.len() && !"aeiouAEIOU".contains(chars[vowel_pos]) {
        vowel_pos += 1;
    }
    
    // If no vowel was found, move the entire word
    if vowel_pos >= chars.len() {
        return format!("{}ay", text);
    }
    
    // Rearrange the word according to Pig Latin rules
    let (consonants, rest) = text.split_at(vowel_pos);
    format!("{}{}ay", rest, consonants)
}