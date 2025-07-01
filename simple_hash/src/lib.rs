use std::collections::HashMap;

/// Counts the frequency of each word in the input slice
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut frequency_map = HashMap::new();
    
    for &word in words {
        // Get the current count or 0 if the word isn't in the map yet
        let count = frequency_map.entry(word).or_insert(0);
        // Increment the count
        *count += 1;
    }
    
    frequency_map
}

/// Returns the number of distinct words in the frequency map
pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_frequency() {
        let words = ["hello", "world", "hello", "rust"];
        let frequency = word_frequency_counter(&words);
        
        assert_eq!(frequency.get("hello"), Some(&2));
        assert_eq!(frequency.get("world"), Some(&1));
        assert_eq!(frequency.get("rust"), Some(&1));
        assert_eq!(frequency.get("missing"), None);
    }
    
    #[test]
    fn test_distinct_words() {
        let mut map = HashMap::new();
        map.insert("hello", 2);
        map.insert("world", 1);
        
        assert_eq!(nb_distinct_words(&map), 2);
    }
}