#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 || self.v == 1 {
            return None;
        }

        let current = *self; 

        if self.v % 2 == 0 {
            self.v /= 2;
        } else {
            self.v = self.v * 3 + 1;
        }

        Some(current)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 || n == 1 {
        return 0;
    }
    Collatz::new(n).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(collatz(0), 0);
        assert_eq!(collatz(1), 0);
        assert_eq!(collatz(4), 2);
        assert_eq!(collatz(5), 5);
        assert_eq!(collatz(6), 8);
        assert_eq!(collatz(7), 16);
        assert_eq!(collatz(12), 9);
    }
    #[test]
    fn test_collatz_large() {
        assert_eq!(collatz(100), 25);
        assert_eq!(collatz(1000), 111);
        assert_eq!(collatz(10000), 29);
        assert_eq!(collatz(100000), 128);
        assert_eq!(collatz(1000000), 152);
    }
    #[test]
    fn test_collatz_edge_cases() {
        assert_eq!(collatz(2), 1);
        assert_eq!(collatz(3), 7);
        assert_eq!(collatz(8), 3);
        assert_eq!(collatz(9), 19);
        assert_eq!(collatz(10), 6);
    }
}