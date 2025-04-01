pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0/5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freezing_point() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_absolute_zero() {
        assert!((fahrenheit_to_celsius(-459.67) - (-273.15)).abs() < 0.01);
        assert!((celsius_to_fahrenheit(-273.15) - (-459.67)).abs() < 0.01);
    }

    #[test]
    fn test_boiling_point() {
        assert!((fahrenheit_to_celsius(212.0) - 100.0).abs() < 0.01);
    }
}
