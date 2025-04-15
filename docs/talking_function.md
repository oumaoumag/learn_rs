# Talking Function in Rust

This document explains the implementation of a `talking` function that simulates a conversation with a computer, following specific response rules based on the input pattern.

## Function Specification

The `talking` function takes a string slice as input and returns a String response according to these rules:

1. If you yell a question (all capital letters ending with "?"), it responds: "Quiet, I am thinking!"
2. If you yell (all capital letters), it responds: "There is no need to yell, calm down!"
3. If you ask a question (ending with "?"), it responds: "Sure."
4. If you don't say anything (empty or whitespace input), it responds: "Just say something!"
5. For any other input, it responds: "Interesting"

## Implementation with Pattern Matching

```rust
pub fn talking(message: &str) -> String {
    // Check if the message is empty or just whitespace
    if message.trim().is_empty() {
        return String::from("Just say something!");
    }

    // Check if the message contains any letters
    let has_letters = message.chars().any(|c| c.is_alphabetic());
    
    // Check if all letters are uppercase (yelling)
    let is_yelling = has_letters && message.chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| c.is_uppercase());
    
    // Check if the message is a question
    let is_question = message.trim().ends_with("?");
    
    // Pattern match on the combination of conditions
    match (is_yelling, is_question) {
        (true, true) => String::from("Quiet, I am thinking!"),
        (true, false) => String::from("There is no need to yell, calm down!"),
        (false, true) => String::from("Sure."),
        (false, false) => String::from("Interesting"),
    }
}
```

## Pattern Matching Explanation

This implementation uses pattern matching in two key ways:

1. **Conditional Logic**: We extract three boolean conditions from the input:
   - Is the input empty?
   - Is the input yelling (all capital letters)?
   - Is the input a question (ends with "?")?

2. **Tuple Pattern Matching**: We create a tuple of `(is_yelling, is_question)` and use pattern matching to handle all four possible combinations:
   ```rust
   match (is_yelling, is_question) {
       (true, true) => String::from("Quiet, I am thinking!"),
       (true, false) => String::from("There is no need to yell, calm down!"),
       (false, true) => String::from("Sure."),
       (false, false) => String::from("Interesting"),
   }
   ```

This approach is more elegant and readable than using nested if-else statements.

## Testing Examples

```rust
fn main() {
    println!("{}", talking("Hello")); // "Interesting"
    println!("{}", talking("Is everything ok with you?")); // "Sure."
    println!("{}", talking("LEAVE ME ALONE!")); // "There is no need to yell, calm down!"
    println!("{}", talking("HOW ARE YOU?")); // "Quiet, I am thinking!"
    println!("{}", talking("")); // "Just say something!"
    println!("{}", talking("   ")); // "Just say something!"
}
```

## Edge Cases Handled

1. **Mixed Case with Question**: "Hello?" is treated as a question, not yelling
2. **Numbers and Symbols**: "123" or "!!!" are neither yelling nor questions, so they get "Interesting"
3. **Empty Input**: Both "" and "   " (just spaces) are handled as empty input
4. **Capital Letters with No Alphabetic Characters**: "123!" is not considered yelling since it has no letters
