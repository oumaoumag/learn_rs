# ROT Cipher Documentation

## What is a ROT Cipher?

Imagine you have a secret code with your friends. You decide that every letter should be moved forward in the alphabet by a certain number of steps. This is called a "ROT" cipher, which stands for "rotation."

For example, if you decide to move every letter forward by 1 step:
- 'a' becomes 'b'
- 'b' becomes 'c'
- 'z' wraps around and becomes 'a'

The most famous ROT cipher is ROT13, where each letter is shifted 13 places. Since the English alphabet has 26 letters, applying ROT13 twice returns the original text - it's its own inverse! This makes it perfect for hiding spoilers or puzzle answers.

## How Our `rotate` Function Works

Our `rotate` function takes a message and a secret number (called a "key"), and it shifts each letter in the message by that number.

### The Function Explained Simply

```rust
pub fn rotate(input: &str, key: i8) -> String {
    // Make sure our key is a positive number between 0-25
    let normalized_key = (((key % 26) + 26) % 26) as u8;

    // Process each letter in the input
    input.chars().map(|c| {
        match c {
            // If it's a lowercase letter (a-z)
            'a'..='z' => {
                let base = b'a';
                let rotated = (((c as u8 - base) + normalized_key) % 26) + base;
                rotated as char
            },
            // If it's an uppercase letter (A-Z)
            'A'..='Z' => {
                let base = b'A';
                let rotated = (((c as u8 - base) + normalized_key) % 26) + base;
                rotated as char
            },
            // If it's not a letter (like numbers or punctuation), don't change it
            _ => c
        }
    }).collect()
}
```

### Step-by-Step Explanation (Like Talking to a Child)

#### 1. Normalizing the Key

First, we need to make sure our secret number (key) makes sense:

```rust
let normalized_key = (((key % 26) + 26) % 26) as u8;
```

Imagine the alphabet has 26 spaces in a circle. If I tell you to move forward 27 spaces, that's the same as moving 1 space (because you go around the circle once and then 1 more). If I tell you to move backward 1 space (key = -1), that's the same as moving forward 25 spaces.

This line makes sure our key is a positive number between 0 and 25, no matter what number we start with.

#### 2. Processing Each Character

```rust
input.chars().map(|c| { ... }).collect()
```

This is like taking each letter from our message one by one, doing something to it, and then putting all the letters back together.

Let's break down this code:

- `input.chars()` - This converts our string into an iterator of individual characters. Think of it like taking a word and splitting it into separate letters.

- `.map(|c| { ... })` - This is a special function that applies a transformation to each character. The `|c|` part is like saying "for each character, which I'll call 'c', do the following..."

- `.collect()` - After we've transformed each character, this gathers all the transformed characters back into a new string.

This pattern is very common in Rust for processing strings character by character. It's like an assembly line where each letter goes in, gets changed, and then all the changed letters are put back together at the end.

#### 3. Pattern Matching

```rust
match c {
    'a'..='z' => { ... },
    'A'..='Z' => { ... },
    _ => c
}
```

This is like sorting our letters into three piles:
- Lowercase letters (a to z)
- Uppercase letters (A to Z)
- Everything else (numbers, spaces, punctuation)

We'll change the letters, but leave everything else alone.

Pattern matching is one of Rust's most powerful features. It's like having a smart sorting machine that can recognize different types of items and handle each type differently.

Let's understand each part:

- `match c { ... }` - This says "look at the character 'c' and decide what to do based on what it is"

- `'a'..='z' => { ... }` - This pattern means "if 'c' is any lowercase letter from 'a' to 'z', do this...". The `..=` creates an inclusive range.

- `'A'..='Z' => { ... }` - Similarly, this handles uppercase letters.

- `_ => c` - The underscore `_` is a wildcard that matches anything else. It's like saying "for any other character, just keep it as is".

Pattern matching is more readable and safer than using multiple if-else statements because the compiler ensures you've handled all possible cases.

#### 4. Rotating Lowercase Letters

```rust
'a'..='z' => {
    let base = b'a';
    let rotated = (((c as u8 - base) + normalized_key) % 26) + base;
    rotated as char
},
```

Let's say we have the letter 'c' and our key is 3:

1. We need to know how far 'c' is from the start of the alphabet:
   - 'a' is position 0
   - 'b' is position 1
   - 'c' is position 2

   In the code, we do this by: `c as u8 - base`
   (This converts 'c' to its number code and subtracts the code for 'a')

   The `b'a'` syntax is Rust's way of getting the ASCII value of the character 'a', which is 97. So `base` equals 97.
   When we convert 'c' to `u8` (an 8-bit unsigned integer), we get its ASCII value, which is 99.
   So `c as u8 - base` is `99 - 97 = 2`, which is the position of 'c' in the alphabet (0-indexed).

2. We add our secret key to move forward:
   - Position 2 + key 3 = position 5

   In the code: `+ normalized_key`

3. If we go past 'z', we need to wrap around to 'a':
   - If we get position 26, that should be position 0 ('a')
   - If we get position 27, that should be position 1 ('b')

   In the code: `% 26` (this is the "remainder" operation)

   For example, if our position after adding the key is 27, then `27 % 26 = 1`, so we wrap around to position 1 ('b').

4. Now we need to convert back to a letter:
   - Position 5 + the code for 'a' = the code for 'f'

   In the code: `+ base`

   We add back the ASCII value of 'a' (97) to convert from a position (0-25) to an ASCII value.
   So position 5 + 97 = 102, which is the ASCII value for 'f'.

5. Finally, we turn the number code back into a letter:

   In the code: `rotated as char`

   This converts the ASCII value (102) back to a character ('f').

So 'c' with a key of 3 becomes 'f'.

#### 5. Rotating Uppercase Letters

The process for uppercase letters (A-Z) is exactly the same, except we use 'A' as our starting point instead of 'a'.

#### 6. Leaving Non-Letters Unchanged

```rust
_ => c
```

This means "for anything else, just keep it the same." So numbers, spaces, and punctuation don't change.

## Examples

1. With key = 1:
   - "abc" becomes "bcd"
   - "ABC" becomes "BCD"
   - "abc123" becomes "bcd123" (numbers stay the same)

2. With key = 13 (this is the famous ROT13):
   - "hello" becomes "uryyb"
   - "HELLO" becomes "URYYB"

3. With key = -1 (rotating backward):
   - "b" becomes "a"
   - "a" becomes "z" (wraps around)

4. With key = 26 (full rotation):
   - "abc" becomes "abc" (we've gone all the way around the alphabet)

## Alternative Implementations

There are many ways to implement a ROT cipher. Here's a simpler version that doesn't use pattern matching but is easier to understand for beginners:

```rust
pub fn rotate_simple(input: &str, key: i8) -> String {
    // Convert key to a positive value between 0-25
    let normalized_key = (((key % 26) + 26) % 26) as u8;

    // Create a new string to store the result
    let mut result = String::with_capacity(input.len());

    // Process each character
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            // Handle lowercase letters
            let rotated = (((c as u8 - b'a') + normalized_key) % 26) + b'a';
            result.push(rotated as char);
        } else if c.is_ascii_uppercase() {
            // Handle uppercase letters
            let rotated = (((c as u8 - b'A') + normalized_key) % 26) + b'A';
            result.push(rotated as char);
        } else {
            // Non-alphabetic characters remain unchanged
            result.push(c);
        }
    }

    result
}
```

This version uses a traditional for loop instead of functional programming, which might be easier to follow for those new to Rust.

## Why This Is Useful

ROT ciphers were used for simple secret messages, but they're not very secure because there are only 25 possible keys to try. ROT13 is sometimes used to hide spoilers or answers because it's easy to apply and reverse.

In our modern world, we use much more complex encryption, but ROT ciphers are a fun way to learn about the basic principles of encryption.

## Historical Context

The ROT cipher is one of the oldest and simplest encryption techniques. The most famous version is the Caesar cipher (ROT3), named after Julius Caesar who reportedly used it to communicate with his generals.

Despite its simplicity, the ROT cipher introduces important concepts in cryptography:

1. **Substitution**: Replacing one character with another
2. **Key-based encryption**: Using a number (the key) to determine how to transform the text
3. **Modular arithmetic**: Using the remainder operation to wrap around the alphabet

## Fun Facts

- The Caesar cipher is just a ROT cipher with a key of 3!
- ROT13 has the special property that applying it twice returns the original text
- In the early days of the internet, ROT13 was commonly used in forums to hide spoilers or offensive content
- The ROT cipher is a monoalphabetic substitution cipher, meaning each letter is always replaced by the same letter throughout the message
