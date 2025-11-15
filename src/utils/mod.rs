//! Utility functions and helpers
//!
//! This module contains various utility functions that don't belong
//! to a specific domain but are used across the application.

pub mod validation;

// Re-export commonly used utilities
pub use validation::{
    parse_file_and_nodes_from_url, parse_file_key_from_url, parse_node_id_from_url,
    parse_page_list, validate_file_key, validate_regex_pattern, validate_token,
};
