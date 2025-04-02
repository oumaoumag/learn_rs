pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    km_h * (5.0 / 18.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_speed() {
        assert_eq!(km_per_hour_to_meters_per_second(0.0), 0.0);
    }

    #[test]
    fn test_common_speeds() {
        // Test 36 km/h = 10 m/s
        assert!((km_per_hour_to_meters_per_second(36.0) - 10.0).abs() < f64::EPSILON);
        
        // Test 72 km/h = 20 m/s
        assert!((km_per_hour_to_meters_per_second(72.0) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_precise_conversion() {
        // Test 1 km/h ≈ 0.277778 m/s
        assert!((km_per_hour_to_meters_per_second(1.0) - 0.277778).abs() < 0.000001);
    }

    #[test]
    fn test_negative_speed() {
        // Test -90 km/h = -25 m/s
        assert!((km_per_hour_to_meters_per_second(-90.0) - (-25.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn test_large_numbers() {
        // Test 360 km/h = 100 m/s
        assert!((km_per_hour_to_meters_per_second(360.0) - 100.0).abs() < f64::EPSILON);
    }
}
