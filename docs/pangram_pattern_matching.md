# Pangram Checker with Pattern Matching in Rust

A pangram is a sentence which uses every letter of the alphabet at least once. This document explains how to implement a pangram checker in Rust using pattern matching.

## Implementation

```rust
fn is_pangram(text: &str) -> bool {
    // Convert to lowercase to simplify checking
    let text = text.to_lowercase();
    
    // Create a set of all lowercase ASCII letters (a-z)
    let mut letters = [false; 26];
    
    // Check each character in the string
    for c in text.chars() {
        // Pattern match to handle only alphabetic characters
        match c {
            'a'..='z' => {
                // Convert char to index (0-25) and mark as seen
                let index = (c as u8 - b'a') as usize;
                letters[index] = true;
            },
            _ => continue, // Skip non-alphabetic characters
        }
    }
    
    // Check if all letters have been seen
    letters.iter().all(|&seen| seen)
}
```

## Pattern Matching Tutorial

Pattern matching is one of Rust's most powerful features. In our pangram checker, we're using it to elegantly handle character classification and processing.

### The Match Expression

The core of pattern matching in Rust is the `match` expression. It allows us to compare a value against a series of patterns and execute code based on which pattern matches.

```rust
match c {
    'a'..='z' => { /* code for lowercase letters */ },
    _ => continue, // Skip non-alphabetic characters
}
```

### Range Patterns

In our solution, we use a range pattern `'a'..='z'` which matches any character from 'a' to 'z' inclusive. This is a concise way to check if a character is a lowercase letter.

Range patterns are extremely useful when you need to match values within a specific range. The syntax `start..=end` includes both the start and end values.

### The Wildcard Pattern

The underscore `_` is a wildcard pattern that matches any value but doesn't bind to it. We use it to skip any character that isn't a lowercase letter:

```rust
_ => continue,
```

### Alternative Approach with Pattern Guards

We could also use pattern guards for more complex conditions:

```rust
match c {
    c if c.is_ascii_lowercase() => {
        let index = (c as u8 - b'a') as usize;
        letters[index] = true;
    },
    _ => continue,
}
```

### Functional Programming with Iterators

The final check uses functional programming style with iterators:

```rust
letters.iter().all(|&seen| seen)
```

This pattern is common in Rust - we transform a collection and then apply a condition to all elements.

### Testing the Function

You could test this function with:

```rust
fn main() {
    let test1 = "The quick brown fox jumps over the lazy dog";
    let test2 = "Hello, world!";
    
    println!("'{}' is a pangram: {}", test1, is_pangram(test1));
    println!("'{}' is a pangram: {}", test2, is_pangram(test2));
}
```

### Optimizations

This solution is already efficient with O(n) time complexity where n is the length of the input string. We only need to scan the string once and use constant space (the boolean array).

For very large strings, you could add an early return once all 26 letters are found:

```rust
// Add this after marking a letter as seen
if letters.iter().all(|&seen| seen) {
    return true;
}
```

This pattern matching approach gives us a clean, readable solution that elegantly handles the problem requirements.
