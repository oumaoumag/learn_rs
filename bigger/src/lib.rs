use std::collections::HashMap;

/// Returns the biggest positive number in the HashMap
/// If no positive numbers are found, returns 0
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values()
        .filter(|&&value| value > 0)  // Filter for positive values only
        .max()                        // Find the maximum value
        .copied()                     // Convert from &i32 to i32
        .unwrap_or(0)                 // Return 0 if no positive values found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_bigger() {
        let hash = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", 333),
            ("Katie", 334),
            ("Robert", 14),
        ]);
        assert_eq!(bigger(hash), 334);
    }

    #[test]
    fn test_bigger_with_negative() {
        let hash = HashMap::from_iter([
            ("Daniel", 122),
            ("Ashley", -333),
            ("Katie", 334),
            ("Robert", -14),
        ]);
        assert_eq!(bigger(hash), 334);
    }

    #[test]
    fn test_bigger_all_negative() {
        let hash = HashMap::from_iter([
            ("Daniel", -122),
            ("Ashley", -333),
            ("Katie", -334),
            ("Robert", -14),
        ]);
        assert_eq!(bigger(hash), 0);
    }

    #[test]
    fn test_bigger_empty() {
        let hash: HashMap<&str, i32> = HashMap::new();
        assert_eq!(bigger(hash), 0);
    }
}