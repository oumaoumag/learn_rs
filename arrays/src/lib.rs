// Takes a slice of i32 values and returns their sum
pub fn sum(a: &[i32]) -> i32 {
    let mut total = 0;
    for &num in a {
        total += num;
    }
    total
}

// Returns an array of 32 elements, all with the value 10
pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]  
}