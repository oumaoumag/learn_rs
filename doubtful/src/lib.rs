pub fn doubtful(s: &mut String) {
    s.push('?')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubtful() {
        let mut s = "Hello".to_owned();
        doubtful(&mut s);
        assert_eq!(s, "Hello?");

        let mut s2  = "Godwin".to_owned();
        doubtful(&mut s2);
        assert_eq!(s2, "Godwin?");

    }
}
