//! Global constants for the application
//!
//! This module centralizes all constants to avoid magic numbers/strings
//! and ensure consistency across the codebase.
// Network Constants
/// Figma API base URL
pub const FIGMA_API_BASE: &str = "https://api.figma.com/v1";

/// Figma API token header name
pub const FIGMA_TOKEN_HEADER: &str = "X-Figma-Token";

/// User agent string for HTTP requests
pub const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (Rust)"
);
// Timing Constants
/// Default HTTP request timeout in seconds
pub const HTTP_TIMEOUT_SECS: u64 = 30;

/// Connection pool idle timeout in seconds
pub const POOL_IDLE_TIMEOUT_SECS: u64 = 90;

/// Default cache TTL in seconds (1 hour)
pub const CACHE_TTL_SECS: u64 = 3600;
// Retry Constants
/// Maximum number of retry attempts
pub const MAX_RETRIES: u32 = 3;

/// Initial retry delay in milliseconds
pub const RETRY_INITIAL_DELAY_MS: u64 = 1000;

/// Maximum retry delay in milliseconds
pub const RETRY_MAX_DELAY_MS: u64 = 32000;

/// Retry jitter factor (min, max)
pub const RETRY_JITTER_FACTOR: (f64, f64) = (0.75, 1.25);
// Resource Limits
/// Maximum concurrent HTTP requests
pub const MAX_CONCURRENT_REQUESTS: usize = 50;

/// Maximum connection pool size per host
pub const MAX_IDLE_CONNECTIONS_PER_HOST: usize = 10;

/// Default extraction depth for document traversal
pub const DEFAULT_EXTRACTION_DEPTH: u32 = 5;

/// Memory limit for in-memory processing (MB)
pub const MEMORY_LIMIT_MB: usize = 1024;

/// File size threshold for streaming (MB)
pub const STREAM_THRESHOLD_MB: usize = 100;

/// Memory pool size (MB)
pub const MEMORY_POOL_MB: usize = 256;

/// Cache size in number of entries
pub const CACHE_SIZE: usize = 1000;

/// `SQLite` connection pool size
pub const SQLITE_POOL_SIZE: u32 = 5;
// Performance Constants
/// Estimated overhead per data structure in bytes
pub const STRUCT_OVERHEAD_BYTES: usize = 200;

/// Buffer size for I/O operations
pub const IO_BUFFER_SIZE: usize = 64 * 1024; // 64KB

/// Default image scale factor
pub const DEFAULT_IMAGE_SCALE: f64 = 2.0;

/// Maximum image scale factor
pub const MAX_IMAGE_SCALE: f64 = 4.0;
// File System Constants
/// Application name for configuration paths
pub const APP_NAME: &str = "figma-cli";

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "config.toml";

/// Project configuration file name
pub const PROJECT_CONFIG_FILE: &str = "figma-cli.toml";

/// Cache directory name
pub const CACHE_DIR_NAME: &str = "cache";

/// Data directory name
pub const DATA_DIR_NAME: &str = "data";
// Validation Constants
/// Figma file key regex pattern
pub const FILE_KEY_PATTERN: &str = r"^[a-zA-Z0-9]{22}$";

/// Figma node ID regex pattern
pub const NODE_ID_PATTERN: &str = r"^\d+:\d+$";

/// Minimum token length
pub const MIN_TOKEN_LENGTH: usize = 10;

/// Maximum token length
pub const MAX_TOKEN_LENGTH: usize = 256;
// Error Messages
/// Error message for missing authentication token
pub const ERR_NO_TOKEN: &str = "No authentication token found. Please run 'figma auth login' or set FIGMA_TOKEN environment variable.";

/// Error message for invalid token
pub const ERR_INVALID_TOKEN: &str =
    "Invalid or expired authentication token. Please run 'figma auth login' to refresh.";

/// Error message for rate limiting
pub const ERR_RATE_LIMIT: &str = "API rate limit exceeded. Please wait before retrying.";

/// Error message for network error
pub const ERR_NETWORK: &str = "Network error occurred. Please check your internet connection.";
// Display Constants
/// Spinner characters for progress indication
pub const SPINNER_CHARS: &[char] = &['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

/// Progress bar width in characters
pub const PROGRESS_BAR_WIDTH: usize = 40;

/// Maximum display width for text output
pub const MAX_DISPLAY_WIDTH: usize = 120;
// Environment Variables
/// Environment variable for Figma API token
pub const ENV_FIGMA_TOKEN: &str = "FIGMA_TOKEN";

/// Environment variable for HTTP timeout
pub const ENV_HTTP_TIMEOUT: &str = "FIGMA_HTTP_TIMEOUT";

/// Environment variable for max retries
pub const ENV_MAX_RETRIES: &str = "FIGMA_MAX_RETRIES";

/// Environment variable for log level
pub const ENV_LOG_LEVEL: &str = "FIGMA_LOG_LEVEL";

/// Environment variable for data directory
pub const ENV_DATA_DIR: &str = "FIGMA_DATA_DIR";
// Feature Flags
/// Enable experimental streaming support
pub const FEATURE_STREAMING: bool = true;

/// Enable SIMD JSON parsing
pub const FEATURE_SIMD_JSON: bool = true;

/// Enable connection pooling
pub const FEATURE_CONNECTION_POOL: bool = true;

/// Enable metrics collection
pub const FEATURE_METRICS: bool = false;

/// Enable distributed tracing
pub const FEATURE_TRACING: bool = false;
