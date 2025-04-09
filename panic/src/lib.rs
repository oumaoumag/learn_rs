use std::fs::File;
use std::io;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            panic!("The file '{}' does not exist.", s)
        }
        Err(e) => {
            panic!("Error opening file '{}': {}", s, e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_open_existing_file() {
        // create a temp file
        let filename = "test_file.txt";
        File::create(filename).unwrap();

        //test opening the file
        let file = open_file(filename);
        assert!(file.metadata().unwrap().is_file());

        // Clean up
        fs::remove_file(filename).unwrap();
    }

    #[test]
    #[should_panic(expected = "does not exist")]
    fn test_open_nonexistent_file() {
        open_file("nonexistent_file.txt");
    }
}
