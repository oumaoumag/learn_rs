use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    // Quick check: if lengths are different, they can't be permutations
    if s1.len() != s2.len() {
        return false;
    }
    
    // Count character frequencies in s1
    let mut char_counts = HashMap::new();
    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    
    // Check character frequencies in s2
    for c in s2.chars() {
        match char_counts.get_mut(&c) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    char_counts.remove(&c);
                }
            },
            None => return false, // Character in s2 not found in s1
        }
    }
    
    // If all characters were matched, the HashMap should be empty
    char_counts.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert!(is_permutation("thought", "thougth"));
        assert!(is_permutation("abc", "cab"));
        assert!(is_permutation("", ""));
        assert!(!is_permutation("abc", "abd"));
        assert!(!is_permutation("abc", "abcc"));
        assert!(!is_permutation("abc", "ab"));
    }
}