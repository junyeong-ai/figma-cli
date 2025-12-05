//! Figma API HTTP client

use crate::client::error::Result;
use crate::client::retry::{RetryConfig, retry_with_backoff};
use crate::core::cache::Cache;
use crate::core::errors::Error;
use crate::models::document::FigmaFile;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::Duration;

const FIGMA_API_BASE: &str = "https://api.figma.com/v1";

/// Figma API client
pub struct FigmaClient {
    client: HttpClient,
    token: String,
    retry_config: RetryConfig,
    cache: Option<Arc<Cache>>,
}

impl FigmaClient {
    /// Create a new Figma client with the given access token
    pub fn new(token: String) -> Result<Self> {
        Self::with_timeout(token, 30)
    }

    /// Create a new Figma client with custom timeout
    pub fn with_timeout(token: String, timeout_secs: u64) -> Result<Self> {
        let client = HttpClient::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .map_err(|e| Error::network(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            token,
            retry_config: RetryConfig::default(),
            cache: None,
        })
    }

    /// Create client with custom retry configuration
    pub const fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// Enable caching with the given cache instance
    pub fn with_cache(mut self, cache: Arc<Cache>) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Set authentication token
    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    /// Validate authentication by making a test request
    pub async fn validate_auth(&self) -> Result<UserInfo> {
        let url = format!("{FIGMA_API_BASE}/me");

        let response = retry_with_backoff(
            || async {
                self.client
                    .get(&url)
                    .header("X-Figma-Token", &self.token)
                    .send()
                    .await
                    .map_err(|e| Error::network(format!("Request failed: {e}")))
            },
            self.retry_config,
        )
        .await?;

        if response.status().is_success() {
            response
                .json::<UserInfo>()
                .await
                .map_err(|e| Error::parse(format!("Failed to parse user info: {e}")))
        } else if response.status().as_u16() == 401 {
            Err(Error::Auth("Invalid token".to_string()))
        } else if response.status().as_u16() == 429 {
            let _retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("60");
            Err(Error::RateLimit)
        } else {
            Err(Error::other(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }

    /// Get a Figma file by key with optional depth parameter
    pub async fn get_file(&self, file_key: &str, depth: Option<u32>) -> Result<FigmaFile> {
        if let Some(cache) = &self.cache
            && let Ok(Some(mut cached)) = cache.get_file(file_key, depth)
        {
            tracing::info!("Cache hit for file: {} (depth: {:?})", file_key, depth);
            cached.file_key = file_key.to_string();
            return Ok(cached);
        }

        let url = format!("{FIGMA_API_BASE}/files/{file_key}");

        tracing::info!("Fetching file: {} (depth: {:?})", file_key, depth);

        let response = retry_with_backoff(
            || async {
                let mut request = self
                    .client
                    .get(&url)
                    .query(&[("branch_data", "false")])
                    .header("X-Figma-Token", &self.token);

                if let Some(d) = depth {
                    request = request.query(&[("depth", d.to_string())]);
                }

                request
                    .send()
                    .await
                    .map_err(|e| Error::network(format!("Request failed: {e}")))
            },
            self.retry_config,
        )
        .await?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }

        let body = response
            .text()
            .await
            .map_err(|e| Error::network(format!("Failed to read response: {e}")))?;

        tracing::debug!("Received response body (length: {} bytes)", body.len());

        let jd = &mut serde_json::Deserializer::from_str(&body);
        let mut file: FigmaFile = serde_path_to_error::deserialize(jd).map_err(|e| {
            Error::parse(format!(
                "Failed to parse Figma file at path '{}': {}",
                e.path(),
                e.inner()
            ))
        })?;

        file.file_key = file_key.to_string();

        tracing::info!(
            "Successfully parsed file: {} (version: {})",
            file.name,
            file.version
        );

        if let Some(cache) = &self.cache
            && let Err(e) = cache.put_file(&file, depth)
        {
            tracing::warn!("Failed to cache file: {}", e);
        }

        Ok(file)
    }

    /// Get specific nodes from a file
    pub async fn get_nodes(
        &self,
        file_key: &str,
        node_ids: &[String],
        depth: Option<u32>,
    ) -> Result<NodesResponse> {
        if let Some(cache) = &self.cache
            && let Ok(Some(cached)) = cache.get_nodes(file_key, node_ids, depth)
        {
            tracing::info!(
                "Cache hit for {} nodes from file: {}",
                node_ids.len(),
                file_key
            );
            return serde_json::from_value(cached)
                .map_err(|e| Error::parse(format!("Cache deserialization failed: {e}")));
        }

        let url = format!("{FIGMA_API_BASE}/files/{file_key}/nodes");
        let ids = node_ids.join(",");

        tracing::info!("Fetching {} nodes from file: {}", node_ids.len(), file_key);

        let response = retry_with_backoff(
            || async {
                let mut request = self
                    .client
                    .get(&url)
                    .header("X-Figma-Token", &self.token)
                    .query(&[("ids", ids.as_str())]);

                if let Some(d) = depth {
                    request = request.query(&[("depth", d.to_string())]);
                }

                request
                    .send()
                    .await
                    .map_err(|e| Error::network(format!("Request failed: {e}")))
            },
            self.retry_config,
        )
        .await?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }

        let nodes_response = response
            .json::<NodesResponse>()
            .await
            .map_err(|e| Error::parse(format!("Failed to parse nodes response: {e}")))?;

        if let Some(cache) = &self.cache {
            match serde_json::to_value(&nodes_response) {
                Ok(value) => {
                    if let Err(e) = cache.put_nodes(file_key, node_ids, depth, &value) {
                        tracing::warn!("Failed to cache nodes: {}", e);
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to serialize nodes for cache: {}", e);
                }
            }
        }

        Ok(nodes_response)
    }

    /// Get image URLs for specific nodes
    pub async fn get_images(
        &self,
        file_key: &str,
        node_ids: &[String],
        format: &str,
        scale: f64,
    ) -> Result<ImageResponse> {
        let url = format!("{FIGMA_API_BASE}/images/{file_key}");
        let ids = node_ids.join(",");

        tracing::info!("Requesting images for {} nodes", node_ids.len());

        let response = retry_with_backoff(
            || async {
                self.client
                    .get(&url)
                    .header("X-Figma-Token", &self.token)
                    .query(&[
                        ("ids", ids.as_str()),
                        ("format", format),
                        ("scale", &scale.to_string()),
                    ])
                    .send()
                    .await
                    .map_err(|e| Error::network(format!("Request failed: {e}")))
            },
            self.retry_config,
        )
        .await?;

        if !response.status().is_success() {
            return Err(self.handle_error_response(response).await);
        }

        response
            .json::<ImageResponse>()
            .await
            .map_err(|e| Error::parse(format!("Failed to parse image response: {e}")))
    }

    /// Handle error responses with detailed error info
    async fn handle_error_response(&self, response: reqwest::Response) -> Error {
        let status = response.status();

        if status.as_u16() == 401 {
            return Error::Auth("Invalid or expired token".to_string());
        }

        if status.as_u16() == 403 {
            return Error::auth("Access denied. Check file permissions.");
        }

        if status.as_u16() == 404 {
            return Error::not_found("File not found");
        }

        if status.as_u16() == 429 {
            let _retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .unwrap_or("60");
            return Error::RateLimit;
        }

        // Try to get error message from response body
        match response.text().await {
            Ok(body) => {
                if status.as_u16() == 400 && body.contains("Request too large") {
                    Error::other(
                        "Request too large. Use --depth parameter to limit response size (try --depth 3 or lower)".to_string()
                    )
                } else {
                    Error::other(format!("API error ({status}): {body}"))
                }
            }
            Err(_) => Error::other(format!("API error: {status}")),
        }
    }
}

/// User information response
#[derive(Debug, serde::Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub handle: String,
    #[serde(default)]
    pub img_url: Option<String>,
}

/// Image URLs response from Figma API
#[derive(Debug, serde::Deserialize)]
pub struct ImageResponse {
    pub err: Option<String>,
    pub images: std::collections::HashMap<String, Option<String>>,
}

/// Nodes response from Figma API
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NodesResponse {
    pub name: String,
    #[serde(default)]
    pub nodes: std::collections::HashMap<String, Option<NodeResult>>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct NodeResult {
    pub document: Option<crate::models::document::Node>,
    pub components: Option<serde_json::Value>,
    pub styles: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = FigmaClient::new("figd_test_token".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_custom_retry_config() {
        let client = FigmaClient::new("figd_test_token".to_string())
            .unwrap()
            .with_retry_config(RetryConfig::new().with_max_retries(5));

        assert_eq!(client.retry_config.max_retries, 5);
    }
}
