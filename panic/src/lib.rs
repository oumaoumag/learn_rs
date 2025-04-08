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

