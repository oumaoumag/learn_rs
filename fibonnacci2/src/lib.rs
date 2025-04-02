pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // Iterative calculation
    let mut prev = 0u32;  // F(n-2)
    let mut curr = 1u32;  // F(n-1)
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0, "Fibonacci of 0 should be 0");
        assert_eq!(fibonacci(1), 1, "Fibonacci of 1 should be 1");
        assert_eq!(fibonacci(2), 1, "Fibonacci of 2 should be 1");
        assert_eq!(fibonacci(4), 3, "Fibonacci of 4 should be 3");
        assert_eq!(fibonacci(20), 6765, "Fibonacci of 20 should be 6765");
        assert_eq!(fibonacci(22), 17711, "Fibonacci of 22 should be 17711");
    }
}