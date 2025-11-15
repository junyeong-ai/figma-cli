//! Image processor with base64 support for AI agents

use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::config::Images;

pub struct ImageProcessor {
    client: Client,
    config: Images,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageData {
    Base64(String),
    Url(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResult {
    pub id: String,
    pub name: String,
    pub data: ImageData,
    pub format: String,
    pub scale: f64,
}

#[derive(Debug, Deserialize)]
pub struct FigmaImageResponse {
    pub err: Option<String>,
    pub images: HashMap<String, String>,
}

impl ImageProcessor {
    pub fn new(config: Images) -> Result<Self> {
        Ok(Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()?,
            config,
        })
    }

    /// Process frame images from Figma API
    pub async fn process_frames(
        &self,
        token: &str,
        file_key: &str,
        node_ids: &[String],
        base64: bool,
    ) -> Result<Vec<ImageResult>> {
        if node_ids.is_empty() {
            return Ok(Vec::new());
        }

        // Get image URLs from Figma API
        let image_urls = self.fetch_image_urls(token, file_key, node_ids).await?;

        let mut results = Vec::new();

        for (node_id, url) in image_urls {
            let result = if base64 {
                // Download and convert to base64
                self.process_as_base64(&node_id, &url).await?
            } else {
                // Return URL only
                ImageResult {
                    id: node_id.clone(),
                    name: format!("frame_{node_id}"),
                    data: ImageData::Url(url),
                    format: self.config.format.clone(),
                    scale: f64::from(self.config.scale),
                }
            };

            results.push(result);
        }

        Ok(results)
    }

    /// Fetch image URLs from Figma API
    async fn fetch_image_urls(
        &self,
        token: &str,
        file_key: &str,
        node_ids: &[String],
    ) -> Result<HashMap<String, String>> {
        let ids = node_ids.join(",");
        let url = format!(
            "https://api.figma.com/v1/images/{}?ids={}&format={}&scale={}",
            file_key, ids, self.config.format, self.config.scale
        );

        let response = self
            .client
            .get(&url)
            .header("X-Figma-Token", token)
            .send()
            .await
            .context("Failed to fetch images from Figma")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "Figma API error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
        }

        let figma_response: FigmaImageResponse = response
            .json()
            .await
            .context("Failed to parse Figma image response")?;

        if let Some(err) = figma_response.err {
            anyhow::bail!("Figma API error: {err}");
        }

        Ok(figma_response.images)
    }

    /// Process image as base64
    async fn process_as_base64(&self, node_id: &str, url: &str) -> Result<ImageResult> {
        let bytes = self.download_image(url).await?;
        let base64_str = general_purpose::STANDARD.encode(&bytes);

        Ok(ImageResult {
            id: node_id.to_string(),
            name: format!("frame_{node_id}"),
            data: ImageData::Base64(base64_str),
            format: self.config.format.clone(),
            scale: f64::from(self.config.scale),
        })
    }

    /// Download image from URL
    async fn download_image(&self, url: &str) -> Result<Vec<u8>> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to download image")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download image: {}", response.status());
        }

        response
            .bytes()
            .await
            .map(|b| b.to_vec())
            .context("Failed to read image bytes")
    }
}

/// Helper for AI agent integration
impl ImageResult {
    /// Convert to AI-friendly format
    pub fn to_ai_format(&self) -> serde_json::Value {
        match &self.data {
            ImageData::Base64(data) => {
                serde_json::json!({
                    "type": "image",
                    "source": "base64",
                    "data": data,
                    "id": self.id,
                    "name": self.name,
                    "format": self.format,
                    "scale": self.scale,
                })
            }
            ImageData::Url(url) => {
                serde_json::json!({
                    "type": "image",
                    "source": "url",
                    "url": url,
                    "id": self.id,
                    "name": self.name,
                    "format": self.format,
                    "scale": self.scale,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_result_to_ai_format() {
        let result = ImageResult {
            id: "test-id".to_string(),
            name: "test-frame".to_string(),
            data: ImageData::Base64("abc123".to_string()),
            format: "png".to_string(),
            scale: 2.0,
        };

        let ai_format = result.to_ai_format();
        assert_eq!(ai_format["type"], "image");
        assert_eq!(ai_format["source"], "base64");
        assert_eq!(ai_format["data"], "abc123");
        assert_eq!(ai_format["id"], "test-id");
        assert_eq!(ai_format["format"], "png");
        assert_eq!(ai_format["scale"], 2.0);
    }

    #[test]
    fn test_url_format() {
        let result = ImageResult {
            id: "test-id".to_string(),
            name: "test-frame".to_string(),
            data: ImageData::Url("https://example.com/image.png".to_string()),
            format: "png".to_string(),
            scale: 2.0,
        };

        let ai_format = result.to_ai_format();
        assert_eq!(ai_format["type"], "image");
        assert_eq!(ai_format["source"], "url");
        assert_eq!(ai_format["url"], "https://example.com/image.png");
        assert_eq!(ai_format["id"], "test-id");
    }
}
