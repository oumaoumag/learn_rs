# Narcissistic Number Function Documentation

## Function: `number_logic`

### Purpose
This function determines whether a given number is a narcissistic number (also known as an Armstrong number or a pluperfect digital invariant).

### Definition
A narcissistic number is a number that equals the sum of its own digits, where each digit is raised to the power of the number of digits in the original number.

### Examples
- **9** returns **true** because 9 = 9¹ = 9
- **10** returns **false** because 10 ≠ 1² + 0² = 1
- **153** returns **true** because 153 = 1³ + 5³ + 3³ = 1 + 125 + 27 = 153
- **154** returns **false** because 154 ≠ 1³ + 5³ + 4³ = 1 + 125 + 64 = 190

### Implementation Details

```rust
pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    
    let digit_count = num_str.len() as u32;
    let sum: u64 = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .map(|digit| digit.pow(digit_count))
        .sum();
    
    sum == num as u64
}
```

### Algorithm Explanation
1. **Convert to string**: The number is converted to a string to easily count and extract its digits.
2. **Count digits**: The length of the string gives us the number of digits.
3. **Process each digit**:
   - Iterate through each character in the string
   - Convert each character to a digit
   - Raise each digit to the power of the total digit count
   - Sum all these values
4. **Check equality**: Compare the calculated sum with the original number

### Type Considerations
- The function takes a `u32` (32-bit unsigned integer) as input
- The sum is calculated as a `u64` (64-bit unsigned integer) to prevent overflow
- The original number is converted to `u64` before comparison

### Time Complexity
- O(d) where d is the number of digits in the input number

### Space Complexity
- O(d) for storing the string representation of the number
