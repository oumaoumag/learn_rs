pub fn factorial(num: u64) -> u64 {
    // Handle base cases: 0! = 1 and 1! = 1
    if num <= 1 {
        return 1;
    }

    // Iterative factorial calculation
    let mut result: u64 = 1;
    for i in 2..=num {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1, "Factorial of 0 should be 1");
        assert_eq!(factorial(1), 1, "Factorial of 1 should be 1");
        assert_eq!(factorial(5), 120, "Factorial of 5 should be 120");
        assert_eq!(factorial(10), 3628800, "Factorial of 10 should be 3628800");
        assert_eq!(
            factorial(19),
            121645100408832000,
            "Factorial of 19 should be 121645100408832000"
        );
    }
}