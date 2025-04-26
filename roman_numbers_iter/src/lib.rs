#[derive(Debug, Clone, PartialEq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

#[derive(Debug, Clone, PartialEq)]
pub enum RomanDigit {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut digits = Vec::new();
        while num >= 1000 { digits.push(RomanDigit::M); num -= 1000; }
        while num >= 500 { digits.push(RomanDigit::D); num -= 500; }
        while num >= 100 { digits.push(RomanDigit::C); num -= 100; }
        while num >= 50 { digits.push(RomanDigit::L); num -= 50; }
        while num >= 10 { digits.push(RomanDigit::X); num -= 10; }
        while num >= 5 { digits.push(RomanDigit::V); num -= 5; }
        while num >= 1 { digits.push(RomanDigit::I); num -= 1; }
        RomanNumber(digits)
    }
}

impl Iterator for RomanNumber {
    type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_value = self.to_u32();
        current_value += 1;
        *self = RomanNumber::from(current_value);
        Some(self.clone())
    }
}

impl RomanNumber {
    pub fn to_u32(&self) -> u32 {
        self.0.iter().map(|d| match d {
            RomanDigit::I => 1,
            RomanDigit::V => 5,
            RomanDigit::X => 10,
            RomanDigit::L => 50,
            RomanDigit::C => 100,
            RomanDigit::D => 500,
            RomanDigit::M => 1000,
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_roman_number() {
        let mut number = RomanNumber::from(15);
        assert_eq!(number.to_u32(), 15);
        assert_eq!(number.next().unwrap().to_u32(), 16);
        assert_eq!(number.next().unwrap().to_u32(), 17);
    }
    #[test]
    fn test_roman_number_empty() {
        let mut number = RomanNumber(vec![]);
        assert_eq!(number.to_u32(), 0);
        assert_eq!(number.next().unwrap().to_u32(), 1);
    }
}