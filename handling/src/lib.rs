use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: P, content: &str) {
    let file = OpenOptions::new()
        .create(true)  // Create file if it doesn't exist
        .append(true)  // Open in append mode
        .open(&path)
        .unwrap_or_else(|e| panic!("Failed to open or create file: {}", e));

    writeln!(file, "{}", content)
        .unwrap_or_else(|e| panic!("Failed to write to file: {}", e));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_and_write() {
        let path = "test_create.txt";
        
        // Create new file and write
        open_or_create(path, "First line");
        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(contents.trim(), "First line");

        // Append to existing file
        open_or_create(path, "Second line");
        let contents = fs::read_to_string(path).unwrap();
        assert_eq!(contents.trim(), "First line\nSecond line");

        // Clean up
        fs::remove_file(path).unwrap();
    }

    #[test]
    #[should_panic(expected = "Failed to write to file")]
    fn test_write_to_readonly() {
        let path = "readonly.txt";
        
        // Create file and make it readonly
        fs::write(path, "").unwrap();
        let mut perms = fs::metadata(path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(path, perms).unwrap();

        open_or_create(path, "Should fail");

        // Clean up
        fs::remove_file(path).unwrap_or(());
    }
}
