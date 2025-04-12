pub fn scytale_cipher(message: String, i: u32) -> String {
    let c = i as usize;         // Number of columns
    let n = message.len();      // Message length
   
    if c == 0 {                
        return String::new();
    }
    // Edge case: no columns
    let r = (n + c - 1) / c;   // Number of rows (ceiling division)
    let chars: Vec<char> = message.chars().collect(); // Convert to vector of chars
    let mut result = String::with_capacity(r * c);    
   
    for j in 0..c {            // For each column
        for k in 0..r {        // For each row
            let idx = j + k * c; // Index in original message
            if idx < n {        
                result.push(chars[idx]);
            } else {             
                result.push(' ');
            }
        }
    }
    result
}