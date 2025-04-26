use std::collections::HashMap;
use std::hash::Hash;

pub fn slices_to_map<'a, T: Eq + Hash, U>(keys: &'a [T], values: &'a [U]) -> HashMap<&'a T, &'a U> {
    let mut map = HashMap::new();
    let len = keys.len().min(values.len());
    for i in 0..len {
        map.insert(&keys[i], &values[i]);
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slices_to_map() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        let map = slices_to_map(&keys, &values);
        assert_eq!(map.get(&"Olivia"), Some(&1));
        assert_eq!(map.get(&"Liam"), Some(&3));
        assert_eq!(map.get(&"Emma"), Some(&23));
        assert_eq!(map.get(&"Noah"), Some(&5));
        assert_eq!(map.get(&"James"), Some(&2));
    }
}