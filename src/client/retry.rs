//! Retry logic with exponential backoff

use crate::client::error::Result;
use crate::core::errors::Error;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Copy)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 1000,
            max_delay_ms: 32000,
        }
    }
}

impl RetryConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn with_max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub const fn with_base_delay(mut self, delay_ms: u64) -> Self {
        self.base_delay_ms = delay_ms;
        self
    }
}

/// Retry a future with exponential backoff and jitter
pub async fn retry_with_backoff<F, Fut, T>(mut operation: F, config: RetryConfig) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 0;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // Check if error is retryable
                if !is_retryable(&e) || attempt >= config.max_retries {
                    return Err(e);
                }

                // Calculate delay with exponential backoff
                let delay_ms = if let Some(retry_after) = extract_retry_after(&e) {
                    // Honor Retry-After header
                    retry_after
                } else {
                    // Exponential backoff: base * 2^attempt
                    let backoff = config.base_delay_ms * 2u64.pow(attempt);
                    std::cmp::min(backoff, config.max_delay_ms)
                };

                // Add jitter (±25% randomness)
                let jitter = (delay_ms as f64 * rand::random::<f64>().mul_add(0.5, 0.75)) as u64;

                tracing::warn!(
                    "Request failed (attempt {}/{}): {}. Retrying in {}ms...",
                    attempt + 1,
                    config.max_retries,
                    e,
                    jitter
                );

                sleep(Duration::from_millis(jitter)).await;
                attempt += 1;
            }
        }
    }
}

const fn is_retryable(error: &Error) -> bool {
    matches!(error, Error::RateLimit | Error::Network(_))
}

const fn extract_retry_after(error: &Error) -> Option<u64> {
    if matches!(error, Error::RateLimit) {
        // Default retry after 60 seconds
        Some(60000)
    } else {
        None
    }
}

// We need rand for jitter - add it to Cargo.toml or use a simpler approach
mod rand {
    use std::time::{SystemTime, UNIX_EPOCH};

    pub fn random<T>() -> T
    where
        T: From<f64>,
    {
        // Simple pseudo-random using system time
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        T::from(f64::from(nanos % 10000) / 10000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_retry_success_on_second_attempt() {
        let attempts = Arc::new(Mutex::new(0));
        let attempts_clone = Arc::clone(&attempts);

        let result = retry_with_backoff(
            move || {
                let attempts = Arc::clone(&attempts_clone);
                async move {
                    let mut count = attempts.lock().unwrap();
                    *count += 1;
                    if *count < 2 {
                        Err(Error::network("Temporary failure"))
                    } else {
                        Ok(42)
                    }
                }
            },
            RetryConfig::default().with_base_delay(10),
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*attempts.lock().unwrap(), 2);
    }

    #[tokio::test]
    async fn test_retry_fails_after_max_attempts() {
        let attempts = Arc::new(Mutex::new(0));
        let attempts_clone = Arc::clone(&attempts);

        let result: Result<()> = retry_with_backoff(
            move || {
                let attempts = Arc::clone(&attempts_clone);
                async move {
                    let mut count = attempts.lock().unwrap();
                    *count += 1;
                    Err(Error::network("Persistent failure"))
                }
            },
            RetryConfig::default()
                .with_max_retries(2)
                .with_base_delay(10),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(*attempts.lock().unwrap(), 3);
    }

    #[tokio::test]
    async fn test_non_retryable_error() {
        let attempts = Arc::new(Mutex::new(0));
        let attempts_clone = Arc::clone(&attempts);

        let result: Result<()> = retry_with_backoff(
            move || {
                let attempts = Arc::clone(&attempts_clone);
                async move {
                    let mut count = attempts.lock().unwrap();
                    *count += 1;
                    Err(Error::auth("Invalid token"))
                }
            },
            RetryConfig::default(),
        )
        .await;

        assert!(result.is_err());
        assert_eq!(*attempts.lock().unwrap(), 1);
    }
}
