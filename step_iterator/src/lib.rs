use std::ops::Add;

pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
}

impl<T> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: PartialOrd + Copy + Add<Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }
        let result = self.current;
        self.current = self.current + self.step;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_iterator() {
        let mut iter = StepIterator::new(0, 10, 2);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(8));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), None);
    }
}