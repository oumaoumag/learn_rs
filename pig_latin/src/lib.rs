pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Check if the word starts with a vowel
    let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
    let is_vowel = "aeiou".contains(first_char);

    if is_vowel {
        // Rule 1: If word begins with a vowel, add "ay" to the end
        return format!("{}ay", text);
    }

    // For words starting with consonants
    let chars: Vec<char> = text.chars().collect();
    
    // Find the position of the first vowel
    let mut vowel_pos = 0;
    while vowel_pos < chars.len() && !"aeiouAEIOU".contains(chars[vowel_pos]) {
        vowel_pos += 1;
    }
    
    // Handle the special "qu" case
    if vowel_pos > 0 && vowel_pos < chars.len() - 1 {
        if (chars[vowel_pos] == 'u' || chars[vowel_pos] == 'U') && 
           (chars[vowel_pos-1] == 'q' || chars[vowel_pos-1] == 'Q') {
            vowel_pos += 1; // Include 'u' in the consonant cluster
        }
    }
    
    // If no vowel was found, move the entire word
    if vowel_pos >= chars.len() {
        return format!("{}ay", text);
    }
    
    // Rearrange the word according to Pig Latin rules
    let (consonants, rest) = text.split_at(vowel_pos);
    format!("{}{}ay", rest, consonants)
}