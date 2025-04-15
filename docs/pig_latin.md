# Pig Latin Documentation

## What is Pig Latin?

Pig Latin is a language game where you change English words according to simple rules. It's like a secret code that kids sometimes use for fun!

Here are the rules for turning a word into Pig Latin:

1. If a word begins with a vowel (a, e, i, o, u), just add "ay" to the end.
   - Example: "apple" becomes "appleay"

2. If a word begins with a consonant (any letter that's not a vowel), take all the consonants before the first vowel, move them to the end of the word, and then add "ay".
   - Example: "hello" becomes "ellohay" (we move "h" to the end and add "ay")

3. There's a special rule for words that start with a consonant followed by "qu": move both the consonant and "qu" to the end, then add "ay".
   - Example: "square" becomes "aresquay" (we move "squ" to the end and add "ay")

## Our `pig_latin` Function

Our function takes a word and transforms it into Pig Latin following these rules.

### The Function Explained Simply

```rust
pub fn pig_latin(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Check if the word starts with a vowel
    let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
    let is_vowel = "aeiou".contains(first_char);

    if is_vowel {
        // Rule 1: If word begins with a vowel, add "ay" to the end
        return format!("{}ay", text);
    }

    // For words starting with consonants
    let chars: Vec<char> = text.chars().collect();
    
    // Find the position of the first vowel
    let mut vowel_pos = 0;
    while vowel_pos < chars.len() && !"aeiouAEIOU".contains(chars[vowel_pos]) {
        vowel_pos += 1;
    }
    
    // Handle the special "qu" case
    if vowel_pos > 0 && vowel_pos < chars.len() - 1 {
        if (chars[vowel_pos] == 'u' || chars[vowel_pos] == 'U') && 
           (chars[vowel_pos-1] == 'q' || chars[vowel_pos-1] == 'Q') {
            vowel_pos += 1; // Include 'u' in the consonant cluster
        }
    }
    
    // If no vowel was found, move the entire word
    if vowel_pos >= chars.len() {
        return format!("{}ay", text);
    }
    
    // Rearrange the word according to Pig Latin rules
    let (consonants, rest) = text.split_at(vowel_pos);
    format!("{}{}ay", rest, consonants)
}
```

### Step-by-Step Explanation (Like Talking to a Child)

#### 1. Checking for Empty Words

```rust
if text.is_empty() {
    return String::new();
}
```

First, we check if someone gave us an empty word. If they did, we just return an empty string because there's nothing to change!

#### 2. Checking if the Word Starts with a Vowel

```rust
let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
let is_vowel = "aeiou".contains(first_char);

if is_vowel {
    return format!("{}ay", text);
}
```

Next, we look at the first letter of the word:
- We get the first character with `text.chars().next()`
- We make sure it's lowercase so we can check it easily
- We check if it's a vowel (a, e, i, o, u)

If the word starts with a vowel, we just add "ay" to the end and we're done!
For example, "apple" becomes "appleay".

#### 3. Handling Words that Start with Consonants

For words that don't start with vowels, we need to do more work:

```rust
let chars: Vec<char> = text.chars().collect();
```

We turn the word into a list of characters so we can look at each one easily.

#### 4. Finding the First Vowel

```rust
let mut vowel_pos = 0;
while vowel_pos < chars.len() && !"aeiouAEIOU".contains(chars[vowel_pos]) {
    vowel_pos += 1;
}
```

We start at the beginning of the word and keep going until we find a vowel:
- We check each character to see if it's a vowel
- If it's not a vowel, we move to the next character
- We keep track of our position with `vowel_pos`

This is like going through the word letter by letter until you find a vowel.

#### 5. Handling the Special "qu" Case

```rust
if vowel_pos > 0 && vowel_pos < chars.len() - 1 {
    if (chars[vowel_pos] == 'u' || chars[vowel_pos] == 'U') && 
       (chars[vowel_pos-1] == 'q' || chars[vowel_pos-1] == 'Q') {
        vowel_pos += 1; // Include 'u' in the consonant cluster
    }
}
```

This is a special rule: if we find a 'q' followed by a 'u', we keep them together.
For example, in "square", we want to move "squ" to the end, not just "sq".

We check:
- If the vowel we found is 'u'
- And the letter before it is 'q'
- If so, we include the 'u' with the consonants by moving our position one more step

#### 6. Handling Words with No Vowels

```rust
if vowel_pos >= chars.len() {
    return format!("{}ay", text);
}
```

If we went through the whole word and didn't find any vowels, we just add "ay" to the end of the original word.

#### 7. Rearranging the Word

```rust
let (consonants, rest) = text.split_at(vowel_pos);
format!("{}{}ay", rest, consonants)
```

Finally, we split the word into two parts:
- `consonants`: all the letters from the beginning up to the first vowel (or after 'qu')
- `rest`: all the remaining letters

Then we put them back together in the Pig Latin order:
1. First the `rest` (the part starting with a vowel)
2. Then the `consonants` (the part we moved to the end)
3. Finally, we add "ay"

For example, with "hello":
- `consonants` is "h"
- `rest` is "ello"
- We put them together as "ello" + "h" + "ay" = "ellohay"

## Examples

Let's see how our function transforms different words:

1. "igloo" → "iglooay" (starts with a vowel, so just add "ay")
2. "apple" → "appleay" (starts with a vowel, so just add "ay")
3. "hello" → "ellohay" (move "h" to the end and add "ay")
4. "square" → "aresquay" (move "squ" to the end and add "ay")
5. "xenon" → "enonxay" (move "x" to the end and add "ay")
6. "chair" → "airchay" (move "ch" to the end and add "ay")
7. "queen" → "ueenqay" (move "q" to the end and add "ay")

## Alternative Implementation with Pattern Matching

Here's another way to write the function using pattern matching, which is a powerful feature in Rust:

```rust
pub fn pig_latin_with_pattern_matching(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    // Get the first character
    let first_char = text.chars().next().unwrap().to_lowercase().next().unwrap();
    
    // Use pattern matching to handle different cases
    match first_char {
        // If the word starts with a vowel
        'a' | 'e' | 'i' | 'o' | 'u' => {
            format!("{}ay", text)
        },
        // If the word starts with a consonant
        _ => {
            let chars: Vec<char> = text.chars().collect();
            
            // Find the position of the first vowel
            let mut vowel_pos = 0;
            while vowel_pos < chars.len() && !"aeiouAEIOU".contains(chars[vowel_pos]) {
                vowel_pos += 1;
            }
            
            // Handle the special "qu" case
            if vowel_pos > 0 && vowel_pos < chars.len() - 1 {
                if (chars[vowel_pos] == 'u' || chars[vowel_pos] == 'U') && 
                   (chars[vowel_pos-1] == 'q' || chars[vowel_pos-1] == 'Q') {
                    vowel_pos += 1;
                }
            }
            
            // If no vowel was found, move the entire word
            if vowel_pos >= chars.len() {
                return format!("{}ay", text);
            }
            
            // Rearrange the word
            let (consonants, rest) = text.split_at(vowel_pos);
            format!("{}{}ay", rest, consonants)
        }
    }
}
```

The pattern matching version uses `match` to check if the first character is a vowel, which makes the code more readable and follows Rust's idiomatic style.

## Fun Facts About Pig Latin

- Pig Latin has been around for hundreds of years and is popular among English-speaking children.
- It's not a real language but a language game or "argot" used for fun or to speak secretly.
- There are similar language games in other cultures, like "Verlan" in French.
- The name "Pig Latin" is a joke - it's not related to Latin or pigs!
- Some Pig Latin words have become common slang, like "ixnay" (from "nix" + "ay").

## Why This Is Useful

Learning about Pig Latin and implementing it in code teaches us about:
1. String manipulation and transformation
2. Pattern matching in programming
3. Following specific rules and algorithms
4. Handling special cases in code

Plus, it's just fun to play with words and create secret messages!
