use std::collections::HashMap;

/// Calculates the mean (average) of a list of integers
pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    
    // Sum all values and divide by the count
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

/// Calculates the median (middle value) of a list of integers
pub fn median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    
    // Create a mutable copy of the list so we can sort it
    let mut sorted = list.to_vec();
    sorted.sort();
    
    let len = sorted.len();
    if len % 2 == 0 {
        // Even number of elements: average the two middle values
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        (sorted[mid_left] + sorted[mid_right]) / 2
    } else {
        // Odd number of elements: return the middle value
        sorted[len / 2]
    }
}

/// Calculates the mode (most frequent value) of a list of integers
pub fn mode(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    
    // Count the frequency of each value
    let mut frequency = HashMap::new();
    for &value in list {
        *frequency.entry(value).or_insert(0) += 1;
    }
    
    // Find the value with the highest frequency
    frequency
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mean(&v), 3.857142857142857);
    }

    #[test]
    fn test_median() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(median(&v), 4);
        
        let v2 = [1, 2, 3, 4];
        assert_eq!(median(&v2), 2); // (2+3)/2 = 2.5, which becomes 2 as an i32
    }

    #[test]
    fn test_mode() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mode(&v), 5);
    }
}