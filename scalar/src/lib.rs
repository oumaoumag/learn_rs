// Takes two u8 values and returns their sum.
pub fn sum(a: u8, b: u8) -> u8 {
    a + b
}

// Takes two i16 values and returns their difference.
pub fn diff(a: i16, b: i16) -> i16 {
    a - b
}

// Takes two i8 values and returns their product.
pub fn pro(a: i8, b: i8) -> i8 {
    a * b
}

// Takes two f32 values and returns the quotient.
pub fn quo(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("division by zero")
    }
    a / b
}

// Takes two f32 values and returns the remainder.
pub fn rem(a: f32, b: f32) -> f32 {
    if b == 0.0 {
        panic!("division by zero")
    }
    a % b
}


#[cfg(test)]
mod tests {
    use super::*;

    // Sum tests
    #[test]
    fn test_sum_normal() {
        assert_eq!(sum(234,2), 236);
        assert_eq!(sum(0, 0), 0);
        assert_eq!(sum(255, 0), 255);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(sum(255, 0), 255);
        assert_eq!(sum(100, 100), 200);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_sum_overflow() {
        sum(255, 1);
    }

    // Difference tests
    #[test]
    fn test_diff_normal() {
        assert_eq!(diff(234, 2), 232);
        assert_eq!(diff(0, 0), 0);
        assert_eq!(diff(-100, 50), -150);
    }



    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn test_diff_overflow() {
        diff(-32768, 32766);
    }

    // Product tests
    #[test]
    fn test_ppro_normal() {
        assert_eq!(pro(23, 2), 46);
        assert_eq!(pro(0, 0), 0);
        assert_eq!(pro(-5, 5), -25);
    }

    #[test]
    #[should_panic(expected="attempt to multiply with overflow")]
    fn test_pro_overflow() {
        pro(-128, 2);
    }

    // Quotient tests
    #[test]
    fn test_quo_normal() {
        assert_eq!(quo(22.0, 2.0), 11.0);
        assert_eq!(quo(-128.23, 2.0), -64.115);
        assert_eq!(quo(0.0, 5.0), 0.0);
    }

    #[test]
    #[should_panic]
    fn test_quo_divide_by_zero() {
        quo(22.0, 0.0);
    }

    // Remainder tests
    #[test]
    fn test_rem_normal() {
        assert!((rem(22.0, 2.0) - 0.0).abs() < f32::EPSILON);
        assert!((rem(-128.23, 2.0) - (-0.22999573)).abs() < 0.0001);
        assert_eq!(rem(5.0, 2.0), 1.0);
    }

    #[test]
    #[should_panic]
    fn test_rem_divide_by_zero() {
        rem(22.0, 0.0);
    }
}
