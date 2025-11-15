//! Input validation utilities

use regex::Regex;
use std::sync::OnceLock;

use crate::core::errors::{Error, Result};

static FILE_KEY_REGEX: OnceLock<Regex> = OnceLock::new();
static URL_REGEX: OnceLock<Regex> = OnceLock::new();
static NODE_ID_REGEX: OnceLock<Regex> = OnceLock::new();

fn file_key_regex() -> &'static Regex {
    FILE_KEY_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9]{22,}$").expect("Failed to compile file key regex")
    })
}

fn url_regex() -> &'static Regex {
    URL_REGEX.get_or_init(|| {
        Regex::new(r"figma\.com/(?:file|design)/([a-zA-Z0-9]{22,})")
            .expect("Failed to compile URL regex")
    })
}

fn node_id_regex() -> &'static Regex {
    NODE_ID_REGEX.get_or_init(|| {
        Regex::new(r"node-id=([0-9]+-[0-9]+)").expect("Failed to compile node-id regex")
    })
}

pub fn validate_file_key(file_key: &str) -> Result<()> {
    if file_key.is_empty() {
        return Err(Error::validation("file_key", "File key cannot be empty"));
    }

    if !file_key_regex().is_match(file_key) {
        return Err(Error::validation(
            "file_key",
            format!(
                "Invalid file key format: '{file_key}'. Expected alphanumeric string of at least 22 characters"
            ),
        ));
    }

    Ok(())
}

pub fn parse_file_key_from_url(input: &str) -> Result<String> {
    if input.contains("figma.com") {
        if let Some(captures) = url_regex().captures(input)
            && let Some(file_key) = captures.get(1)
        {
            let key = file_key.as_str().to_string();
            validate_file_key(&key)?;
            return Ok(key);
        }

        return Err(Error::validation(
            "url",
            format!("Could not extract file key from URL: {input}"),
        ));
    }

    validate_file_key(input)?;
    Ok(input.to_string())
}

pub fn validate_token(token: &str) -> Result<()> {
    if token.is_empty() {
        return Err(Error::validation("token", "Token cannot be empty"));
    }

    if !token.starts_with("figd_") {
        return Err(Error::validation(
            "token",
            "Token should start with 'figd_'. Get your token at https://www.figma.com/settings",
        ));
    }

    if token.len() < 10 {
        return Err(Error::validation("token", "Token is too short"));
    }

    Ok(())
}

pub fn parse_page_list(input: &str) -> Vec<String> {
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn validate_regex_pattern(pattern: &str) -> Result<Regex> {
    Regex::new(pattern).map_err(|e| {
        Error::validation(
            "regex_pattern",
            format!("Invalid regex pattern '{pattern}': {e}"),
        )
    })
}

/// Parse node ID from URL or convert hyphenated format to colon format
pub fn parse_node_id_from_url(url: &str) -> Option<String> {
    // Try URL format first
    if let Some(captures) = node_id_regex().captures(url)
        && let Some(node_id) = captures.get(1)
    {
        return Some(node_id.as_str().replace('-', ":"));
    }

    // Check if it's already a node ID (format: number-number or number:number)
    if url
        .chars()
        .all(|c| c.is_ascii_digit() || c == '-' || c == ':')
        && (url.contains(':') || url.contains('-'))
    {
        return Some(url.replace('-', ":"));
    }

    None
}

/// Parse file key and node IDs from a Figma URL
pub fn parse_file_and_nodes_from_url(url: &str) -> Result<(String, Vec<String>)> {
    let file_key = parse_file_key_from_url(url)?;

    let mut node_ids = Vec::new();

    // Look for node-id parameter
    if let Some(node_id) = parse_node_id_from_url(url) {
        node_ids.push(node_id);
    }

    Ok((file_key, node_ids))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_file_key() {
        // Valid keys
        assert!(validate_file_key("abcdefghij1234567890AB").is_ok());
        assert!(validate_file_key("X".repeat(22).as_str()).is_ok());

        // Invalid keys
        assert!(validate_file_key("").is_err());
        assert!(validate_file_key("short").is_err());
        assert!(validate_file_key("special-chars!").is_err());
        assert!(validate_file_key("with spaces").is_err());
    }

    #[test]
    fn test_parse_file_key_from_url() {
        // From URL
        let url = "https://www.figma.com/file/ABC123XYZ456789012345678/Design";
        assert_eq!(
            parse_file_key_from_url(url).unwrap(),
            "ABC123XYZ456789012345678"
        );

        // Direct key
        assert_eq!(
            parse_file_key_from_url("ABC123XYZ456789012345678").unwrap(),
            "ABC123XYZ456789012345678"
        );

        // Invalid
        assert!(parse_file_key_from_url("https://figma.com/file/short").is_err());
    }

    #[test]
    fn test_validate_token() {
        // Valid
        assert!(validate_token("figd_1234567890").is_ok());

        // Invalid
        assert!(validate_token("").is_err());
        assert!(validate_token("invalid").is_err());
        assert!(validate_token("figd_").is_err());
    }

    #[test]
    fn test_parse_page_list() {
        assert_eq!(
            parse_page_list("Page 1, Page 2, Page 3"),
            vec!["Page 1", "Page 2", "Page 3"]
        );
        assert_eq!(parse_page_list("Single"), vec!["Single"]);
        assert_eq!(parse_page_list(""), Vec::<String>::new());
        assert_eq!(parse_page_list("  ,  ,  "), Vec::<String>::new());
    }

    #[test]
    fn test_validate_regex_pattern() {
        assert!(validate_regex_pattern(r"^\d+$").is_ok());
        assert!(validate_regex_pattern(r"[a-z]+").is_ok());
        assert!(validate_regex_pattern(r"[unclosed").is_err());
    }

    #[test]
    fn test_parse_node_id_from_url() {
        // From URL parameter
        assert_eq!(
            parse_node_id_from_url("https://figma.com/file/ABC?node-id=123-456"),
            Some("123:456".to_string())
        );

        // Direct node ID with hyphen
        assert_eq!(
            parse_node_id_from_url("123-456"),
            Some("123:456".to_string())
        );

        // Direct node ID with colon
        assert_eq!(
            parse_node_id_from_url("123:456"),
            Some("123:456".to_string())
        );

        // No node ID
        assert_eq!(parse_node_id_from_url("https://figma.com/file/ABC"), None);
    }

    #[test]
    fn test_parse_file_and_nodes_from_url() {
        let url = "https://www.figma.com/file/ABC123XYZ456789012345678/Design?node-id=123-456";
        let (file_key, node_ids) = parse_file_and_nodes_from_url(url).unwrap();

        assert_eq!(file_key, "ABC123XYZ456789012345678");
        assert_eq!(node_ids, vec!["123:456"]);
    }
}
