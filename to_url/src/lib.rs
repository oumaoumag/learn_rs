pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_url() {
        assert_eq!(to_url("Hello, world!"), "Hello,%20world!");
        assert_eq!(to_url("Hi   there"), "Hi%20%20%20there");
        assert_eq!(to_url("NoSpaces"), "NoSpaces");
        assert_eq!(to_url(""), "");
    }
}