use std::fs::File;
use std::io;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            panic!("No such file or directory")
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_open_existing_file() {
        // Create a temporary file
        let filename = "test_file.txt";
        File::create(filename).unwrap();
        
        // Test opening the file
        let file = open_file(filename);
        assert!(file.metadata().unwrap().is_file());
        
        // Clean up
        fs::remove_file(filename).unwrap();
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn test_open_nonexistent_file() {
        open_file("nonexistent_file.txt");
    }

    // #[test]
    // #[should_panic]
    // fn test_open_directory() {
    //     // Attempting to open a directory should panic
    //     open_file(".");
    // }
}