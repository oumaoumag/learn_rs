pub fn char_length(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_length() {
        assert_eq!(char_length("❤"), 1);
        assert_eq!(char_length("形聲字"), 3);
        assert_eq!(char_length("change"), 6);
        assert_eq!(char_length("😍"), 1);
    }
}