use crate::core::errors::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn from_status_code(status: u16, message: String) -> Error {
    match status {
        401 => Error::auth(message),
        403 => Error::auth(format!("Access denied: {message}")),
        404 => Error::not_found(message),
        429 => Error::RateLimit,
        500..=599 => Error::network(format!("Server error ({status}): {message}")),
        _ => Error::network(format!("HTTP {status}: {message}")),
    }
}

pub fn parse_retry_after(header_value: &str) -> Option<u64> {
    header_value.parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_status_code() {
        let err = from_status_code(401, "Unauthorized".to_string());
        assert!(matches!(err, Error::Auth(_)));

        let err = from_status_code(429, "Rate limit exceeded".to_string());
        assert!(matches!(err, Error::RateLimit));

        let err = from_status_code(404, "Not found".to_string());
        assert!(matches!(err, Error::NotFound(_)));

        let err = from_status_code(500, "Internal server error".to_string());
        assert!(matches!(err, Error::Network(_)));
    }

    #[test]
    fn test_parse_retry_after() {
        assert_eq!(parse_retry_after("60"), Some(60));
        assert_eq!(parse_retry_after("invalid"), None);
        assert_eq!(parse_retry_after("3600"), Some(3600));
    }
}
