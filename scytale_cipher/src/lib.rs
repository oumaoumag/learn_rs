pub fn scytale_cipher(message: &str, i: usize) -> String {
    let c = i;
    let n = message.len();

    if c == 0 || n == 0 {
        return String::new();
    }

    let r = (n + c - 1) / c;

  
    let mut grid = vec![vec![' '; r]; c];

 
    let mut char_iter = message.chars();
    for row in 0..r {
        for col in 0..c {
            if let Some(ch) = char_iter.next() {
                grid[col][row] = ch;
            } else {
                break;
            }
        }
    }

    let mut result = String::new();
    for col in 0..c {
        for row in 0..r {
                if grid[col][row] != ' ' {
                result.push(grid[col][row]);
            }
        }
    }

    result
}