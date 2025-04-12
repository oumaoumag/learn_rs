pub fn is_pangram(text: &str) -> bool {
    let text = text.to_lowercase();

    let mut letters = [false; 26];

    for c in text.chars() {
        match c {
            'a'..='z' => {
                let index = (c as u8 - b'a') as usize;
                letters[index] = true
            },
            _ => continue,
        }
    }
    letters.iter().all(|&seen| seen)
}