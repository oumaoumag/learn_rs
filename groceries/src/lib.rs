pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(slice: &[String], index: usize) -> &str {
    &slice[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut vec = vec!["apple".to_string(), "banana".to_string()];
        insert(&mut vec, "orange".to_string());
        assert_eq!(vec, vec!["apple".to_string(), "banana".to_string(), "orange".to_string()]);
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_at_index() {
        let vec = vec!["apple".to_string(), "banana".to_string(), "orange".to_string()];
        assert_eq!(at_index(&vec, 0), "apple");
        assert_eq!(at_index(&vec, 1), "banana");
        assert_eq!(at_index(&vec, 2), "orange");
    }
}