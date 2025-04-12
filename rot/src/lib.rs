pub fn rotate (input: &str, key: i8) -> String {
    // Key convertion to a positive value btwn 0-25
  let normalized_key = (((key % 26 ) + 26)% 26) as u8; 

  input.chars().map(|c| {
    match c {
        // matching lowercase letters
        'a'..='z' => {
            let base = b'a';
            let rotated = (((c as u8 - base) + normalized_key) % 26) + base;
            rotated as char
        },
        // Matching uppercase letters
        'A'..='Z' => {
            let base = b'A';
            let rotated = (((c as u8 -base) + normalized_key) % 26) + base;
            rotated as char
        },
        // any characters left are unchanged
        _ => c
    }
  }).collect()
}