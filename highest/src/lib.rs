#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Numbers { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().copied().max()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted_numbers = self.numbers.to_vec();
        sorted_numbers.sort_unstable_by(|a, b| b.cmp(a));
        sorted_numbers.into_iter().take(3).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        assert_eq!(n.list(), &expected);
    }
    #[test]
    fn test_latest() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        assert_eq!(n.latest(), Some(70));
    }
    #[test]
    fn test_highest() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        assert_eq!(n.highest(), Some(500));
    }
    #[test]
    fn test_highest_three() {
        let expected = [30, 500, 20, 70];
        let n = Numbers::new(&expected);
        assert_eq!(n.highest_three(), vec![500, 70, 30]);
    }
    #[test]
    fn test_highest_three_empty() {
        let expected: [u32; 0] = [];
        let n = Numbers::new(&expected);
        assert_eq!(n.highest_three(), vec![]);
    }
}