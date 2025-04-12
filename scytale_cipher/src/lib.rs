pub fn scytale_cipher(message: String, i: u32) -> String {
    let c = i as usize;         // Number of columns
    let n = message.len();      // Length of the message

    if c == 0 {
        return String::new();
    }

    let r = (n + c - 1) / c;

    // Create a 2D grid to hold the characters
    let mut grid = vec![vec![' '; r]; c];

    // Fill the grid row by row
    let mut idx = 0;
    for row in 0..r {
        for col in 0..c {
            if idx < n {
                grid[col][row] = message.chars().nth(idx).unwrap();
                idx += 1;
            }
        }
    }

    // Read the grid column by column to get the encrypted message
    let mut result = String::new();
    for col in 0..c {
        for row in 0..r {
            if col * r + row < n {
                result.push(grid[col][row]);
            }
        }
    }

    result
}