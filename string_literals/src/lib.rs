pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).expect("Pattern not found in string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let s = "";
        assert!(is_empty(s), "The string should be empty");
    }

    #[test]
    fn test_non_empty_string() {
        let s = "Godwin";
        assert!(!is_empty(s), "The string should not be empty");
    }

    #[test]
    fn test_has_ascii() {
        let s = "ouma";
        assert!(is_ascii(s), "The string should have ascii characters");
    }

    #[test]
    fn test_non_ascii() {
        let s = "🔥";
        assert!(!is_ascii(s), "The string should not have ascii charaters");
    }

    #[test]
    fn test_contains_part() {
        let s = "Godwin";
        let pat = "win";
        assert!(contains(s, pat), "The string should contain the substring");
    }
    
    #[test]
    fn test_not_contains() {
        let s = "Godwin";
        let pat = "man";
        assert!(!contains(s, pat), "The string should not contain the substring");
    }

    #[test]
    fn test_splits() {
        let s = "Godwin Ouma";
        let index = 6;
        let result = split_at(s, index);
        let expected = ("Godwin", " Ouma") ;
        assert_eq!(result, expected, "The string should split at space - index 6");
    }

    #[test]
    fn test_finds() {
        let s = "Godwin Ouma";
        let pat = 'o';
        let result = find(s, pat);
        let expected = 1 ;
        assert_eq!(result, expected, "The string should split at space - index 6");
    }
}
