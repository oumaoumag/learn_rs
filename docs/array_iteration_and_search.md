# Rust Array Iteration and Search Functions

## How to Iterate Through Arrays in Rust

There are several ways to iterate through arrays in Rust. Here are the most common approaches:

### 1. Using a `for` loop (most common)

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Iterate through elements
    for number in numbers {
        println!("Value: {}", number);
    }
}
```

### 2. Using a `for` loop with indices

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Iterate with indices
    for i in 0..numbers.len() {
        println!("Index: {}, Value: {}", i, numbers[i]);
    }
}
```

### 3. Using `iter()` method

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Using iter() method
    for number in numbers.iter() {
        println!("Value: {}", number);
    }
}
```

### 4. Using `enumerate()` to get both index and value

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Get both index and value
    for (index, value) in numbers.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}
```

### 5. Using `while` loop with an index

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < numbers.len() {
        println!("Value: {}", numbers[index]);
        index += 1;
    }
}
```

### 6. Mutable iteration (if you need to modify elements)

```rust
fn main() {
    let mut numbers = [1, 2, 3, 4, 5];

    // Iterate with mutable references
    for number in numbers.iter_mut() {
        *number *= 2; // Double each number
    }

    println!("Modified array: {:?}", numbers);
}
```

### Key Points to Remember

- Use `for item in array` for simple iteration when you don't need the original values after the loop
- Use `for item in array.iter()` when you need to keep the original array intact
- Use `for item in array.iter_mut()` when you need to modify the array elements
- Use `for (index, item) in array.iter().enumerate()` when you need both the index and value
- The `..` operator creates a range (e.g., `0..5` is 0, 1, 2, 3, 4)
- The `..=` operator creates an inclusive range (e.g., `0..=5` is 0, 1, 2, 3, 4, 5)

These patterns work for arrays, slices, and most other collection types in Rust.

## Understanding Search Function Types

The function signature:

```rust
pub fn search(array: &[i32], key: i32) -> Option<usize>
```

Let's break down the types:

1. `array: &[i32]` - This is a reference to a slice of 32-bit integers (`i32`).
   - `&` indicates it's a reference (borrowing the data)
   - `[i32]` is a slice type, which is a view into a contiguous sequence of `i32` values
   - Unlike an array, a slice doesn't have a fixed size known at compile time

2. `key: i32` - This is a 32-bit signed integer parameter

3. `-> Option<usize>` - This is the return type:
   - `Option` is an enum that represents either some value (`Some(value)`) or no value (`None`)
   - `<usize>` indicates that the "some" variant contains an unsigned size type, which is typically used for array indices in Rust
   - This return type suggests the function is looking for `key` in the `array` and will return the index if found (`Some(index)`) or `None` if not found

## Correcting a Search Function Implementation

The following implementation has syntax errors:

```rust
pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut index: usize = 0;

    for (i, value) in array.iter().enumerate() {
        if value == &key {
            index += i
        }
    }
    Option<index>  // This line is incorrect
}
```

The issue is with the last line `Option<index>`. This is not valid Rust syntax. To return an `Option<usize>`, you need to use either `Some(index)` or `None`.

A corrected version would be:

```rust
pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, value) in array.iter().enumerate() {
        if *value == key {
            return Some(i);  // Return the index when found
        }
    }
    None  // Return None if not found
}
```

This version correctly returns the index of the first occurrence of the key, or None if the key isn't found.

## Understanding Ordinal Numbers

The ordinal number for a given cardinal number refers to the number that indicates position or order in a sequence, as opposed to the cardinal number which indicates quantity.

Here's a simple explanation:

- **Cardinal numbers** tell "how many" (1, 2, 3, 4, 5...)
- **Ordinal numbers** tell "position" or "order" (1st, 2nd, 3rd, 4th, 5th...)

For example:
- The cardinal number 1 corresponds to the ordinal number 1st (first)
- The cardinal number 2 corresponds to the ordinal number 2nd (second)
- The cardinal number 3 corresponds to the ordinal number 3rd (third)
- The cardinal number 4 corresponds to the ordinal number 4th (fourth)

In English, we form most ordinal numbers by adding "th" to the cardinal number, with special cases for 1 (1st), 2 (2nd), and 3 (3rd). Numbers ending in 1, 2, or 3 follow this pattern except when the second-to-last digit is 1 (like 11, 12, 13), which all use "th" (11th, 12th, 13th).

In programming, ordinal numbers are often used for indexing (though many languages use zero-based indexing), while cardinal numbers are used for counting quantities.

## Using Pattern Matching to Convert Cardinal to Ordinal Numbers

You can use Rust's pattern matching capabilities to convert cardinal numbers to their ordinal forms. Here's how you could implement this:

```rust
pub fn to_ordinal(n: u32) -> String {
    let suffix = match (n % 100, n % 10) {
        (11..=13, _) => "th", // Special case for 11th, 12th, 13th
        (_, 1) => "st",       // Numbers ending in 1 (except 11) -> 1st, 21st, etc.
        (_, 2) => "nd",       // Numbers ending in 2 (except 12) -> 2nd, 22nd, etc.
        (_, 3) => "rd",       // Numbers ending in 3 (except 13) -> 3rd, 23rd, etc.
        _ => "th",            // All other numbers -> 4th, 5th, etc.
    };

    format!("{}{}", n, suffix)
}
```

### How This Works:

1. The function takes a cardinal number (`n`) as input.

2. We use a `match` expression with a tuple pattern `(n % 100, n % 10)` which gives us:
   - The last two digits of the number (n % 100)
   - The last digit of the number (n % 10)

3. The pattern matching rules:
   - If the last two digits are 11, 12, or 13 (`(11..=13, _)`), we use "th" regardless of the last digit
   - If the last digit is 1 (`(_, 1)`), we use "st" (but not for numbers ending in 11)
   - If the last digit is 2 (`(_, 2)`), we use "nd" (but not for numbers ending in 12)
   - If the last digit is 3 (`(_, 3)`), we use "rd" (but not for numbers ending in 13)
   - For all other cases (`_`), we use "th"

4. Finally, we format the result by combining the original number with the appropriate suffix.

### Alternative Approach with Range Patterns:

You could also use range patterns for a slightly different implementation:

```rust
pub fn to_ordinal(n: u32) -> String {
    let suffix = match n % 100 {
        11 | 12 | 13 => "th",
        _ => match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    };

    format!("{}{}", n, suffix)
}
```

This approach first checks if the number ends with 11, 12, or 13, and if not, then checks the last digit.

Pattern matching is a powerful feature in Rust that makes this kind of logic very clean and readable!
