pub fn score(word: &str) -> u64 {
    let mut total_score = 0;

    for c in word.chars() {
        // convert to uppercase for case-insensitive comparison
        let letter = c.to_ascii_uppercase();

        // calculate score based on letter
        let letter_score = match letter {
            'A' | 'E' | 'I' | 'O'| 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1, 
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W'| 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
             _ => 0, // Any other character scores 0
        };
        total_score += letter_score;
    }
    total_score
}