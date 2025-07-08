pub fn scytale_cipher(message: &str, i: usize) -> String {
    // 'c' represents the number of columns, which is given by 'i' (the wrap count).
    let c = i;
    // 'n' is the length of the original message.
    let n = message.len();
    
    if c == 0 || n == 0 {
        return String::new();
    }

    let r = (n + c - 1) / c;

    let mut grid = vec![vec!['\0'; r]; c];

    let mut char_iter = message.chars();
    for row in 0..r {
        for col in 0..c {
            // Check if we still have characters from the message.
            if let Some(ch) = char_iter.next() {
                grid[col][row] = ch;
            } else {
                break;
            }
        }
    }

    // Read the grid column by column to construct the encrypted message.
    // This simulates unwrapping the strip and reading along its length.
    let mut result = String::new();
    for col in 0..c {
        for row in 0..r {
            if grid[col][row] != '\0' {
                result.push(grid[col][row]);
            }
        }
    }

    result
}