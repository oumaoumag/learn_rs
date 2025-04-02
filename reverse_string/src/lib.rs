pub fn rev_str(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_reversal() {
        assert_eq!(rev_str("Hello"), "olleH");
        assert_eq!(rev_str("Rust"), "tsuR");
    }

    #[test]
    fn test_with_spaces_and_punctuation() {
        assert_eq!(rev_str("Hello, world!"), "!dlrow ,olleH");
        assert_eq!(rev_str("I have a nice car!"), "!rac ecin a evah I");
    }

    #[test]
    fn test_with_unicode() {
        assert_eq!(rev_str("água"), "augá");
        assert_eq!(rev_str("ex: this is an example água"), "augá elpmaxe na si siht :xe");
    }

    #[test]
    fn test_empty_and_single_char() {
        assert_eq!(rev_str(""), "");
        assert_eq!(rev_str("a"), "a");
    }
}