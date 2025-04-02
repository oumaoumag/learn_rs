pub fn divide(x: i32, y: i32) -> (i32, i32) {
    (x / y, x % y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_division() {
        assert_eq!(divide(9, 4), (2, 1));  // Matches example
        assert_eq!(divide(10, 5), (2, 0)); // Even division
        assert_eq!(divide(0, 5), (0, 0));  // Zero dividend
    }

    #[test]
    fn test_negative_division() {
        assert_eq!(divide(-9, 4), (-2, -1));
        assert_eq!(divide(9, -4), (-2, 1));
        assert_eq!(divide(-9, -4), (2, -1));
    }

    #[test]
    #[should_panic]
    fn test_division_by_zero() {
        divide(9, 0); // Should panic
    }
}