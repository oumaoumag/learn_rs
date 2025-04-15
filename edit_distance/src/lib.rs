pub fn edit_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();
    
    // Handle edge cases
    if a_len == 0 { return b_len; }
    if b_len == 0 { return a_len; }
    
    // Convert strings to vectors of chars for easier indexing
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    // Create a matrix to store distances
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];
    
    // Initialize first row and column
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }
    
    // Fill the matrix
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i-1].to_lowercase().to_string() == b_chars[j-1].to_lowercase().to_string() { 0 } else { 1 };
            
            matrix[i][j] = std::cmp::min(
                matrix[i-1][j] + 1,                 // deletion
                std::cmp::min(
                    matrix[i][j-1] + 1,             // insertion
                    matrix[i-1][j-1] + cost         // substitution
                )
            );
        }
    }
    
    // Return the bottom-right value
    matrix[a_len][b_len]
}
