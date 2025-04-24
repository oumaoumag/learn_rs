pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = delete_prefix("God", "Godwin");
        assert_eq!(result, Some("win"));
    }
}
