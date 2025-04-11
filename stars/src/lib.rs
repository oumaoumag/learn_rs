pub fn stars(n: u32) -> String {
    // Calculate 2^n
    let count = 2u32.pow(n) as usize;

    // String with 'count' asterisks
    "*".repeat(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stars_() {
        assert_eq!(stars(0), "*");
        assert_eq!(stars(1), "**");
        assert_eq!(stars(3), "********");
        assert_eq!(stars(4), "****************");

        let expected = "*".repeat(32);
        assert_eq!(stars(5), expected);

    }
}