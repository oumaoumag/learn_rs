// fibonacci2/src/lib.rs
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut prev = 0u32;
    let mut curr = 1u32;
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
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(20), 6765);
        assert_eq!(fibonacci(22), 17711);
    }
}