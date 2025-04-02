use std::io;

const RIDDLE: &str = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
const ANSWER: &str = "the letter e";

fn main() {
    let mut tries = 0;
    loop {
        println!("{}", RIDDLE);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        tries += 1;
        if check_answer(&input) {
            println!("Number of trials: {}", tries);
            break;
        }
    }
}

fn check_answer(input: &str) -> bool {
    input.trim().to_lowercase() == ANSWER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_answer() {
        assert!(check_answer("The letter e"));
        assert!(check_answer("the letter e"));
        assert!(check_answer("THE LETTER E"));
        assert!(check_answer("  the letter e  "));
    }

    #[test]
    fn test_incorrect_answer() {
        assert!(!check_answer("I don’t know"));
        assert!(!check_answer("the letter a"));
        assert!(!check_answer(""));
    }
}