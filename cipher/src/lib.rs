#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected = original.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            if c.is_ascii_uppercase() {
                (b'A' + b'Z' - c as u8) as char
            } else {
                (b'a' + b'z' - c as u8) as char
            }
        } else {
            c
        }
    }).collect::<String>();

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_cipher() {
        assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Ok(()));
    }

    #[test]
    fn test_incorrect_cipher() {
        assert_eq!(
            cipher("1Hello 2world!", "svool"),
            Err(CipherError { expected: "1Svool 2dliow!".to_string() })
        );
    }
}
