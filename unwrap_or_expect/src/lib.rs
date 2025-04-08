pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => match server {
            Ok(url) => url.to_string(),
            Err(_) => "WARNING: check the server".to_string(),
        },
        Security::NotFound => match server {
            Ok(url) => url.to_string(),
            Err(msg) => format!("Not found: {}", msg),
        },
        Security::UnexpectedUrl => match server {
            Ok(url) => panic!("{}", url),
            Err(msg) => msg.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Security::Unknown tests
    #[test]
    fn test_unknown_ok() {
        let result = fetch_data(Ok("server1.com"), Security::Unknown);
        assert_eq!(result, "server1.com");
    }

    #[test]
    #[should_panic]
    fn test_unknown_err() {
        fetch_data(Err("error"), Security::Unknown);
    }

    // Security::Message tests
    #[test]
    fn test_message_ok() {
        let result = fetch_data(Ok("server2.com"), Security::Message);
        assert_eq!(result, "server2.com");
    }

    #[test]
    #[should_panic(expected = "ERROR: program stops")]
    fn test_message_err() {
        fetch_data(Err("error"), Security::Message);
    }

    // Security::Warning tests
    #[test]
    fn test_warning_ok() {
        let result = fetch_data(Ok("server3.com"), Security::Warning);
        assert_eq!(result, "server3.com");
    }

    #[test]
    fn test_warning_err() {
        let result = fetch_data(Err("error"), Security::Warning);
        assert_eq!(result, "WARNING: check the server");
    }

    // Security::NotFound tests
    #[test]
    fn test_not_found_ok() {
        let result = fetch_data(Ok("server4.com"), Security::NotFound);
        assert_eq!(result, "server4.com");
    }

    #[test]
    fn test_not_found_err() {
        let result = fetch_data(Err("server5.com"), Security::NotFound);
        assert_eq!(result, "Not found: server5.com");
    }

    // Security::UnexpectedUrl tests
    #[test]
    #[should_panic(expected = "malicious_server.com")]
    fn test_unexpected_url_ok() {
        fetch_data(Ok("malicious_server.com"), Security::UnexpectedUrl);
    }

    #[test]
    fn test_unexpected_url_err() {
        let result = fetch_data(Err("error_msg"), Security::UnexpectedUrl);
        assert_eq!(result, "error_msg");
    }
}