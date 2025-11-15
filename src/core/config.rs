use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use zeroize::Zeroize;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Config {
    pub token: Option<String>,

    #[serde(default)]
    pub extraction: ExtractionConfig,

    #[serde(default)]
    pub http: HttpConfig,

    #[serde(default)]
    pub images: ImageConfig,

    #[serde(default)]
    pub performance: PerformanceConfig,

    #[serde(default)]
    pub cache: CacheConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtractionConfig {
    #[serde(default = "default_depth")]
    pub depth: u32,

    #[serde(default = "default_max_depth")]
    pub max_depth: u32,

    #[serde(default = "default_include_styles")]
    pub styles: bool,

    #[serde(default = "default_include_components")]
    pub components: bool,

    #[serde(default = "default_include_vectors")]
    pub vectors: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HttpConfig {
    #[serde(default = "default_timeout")]
    pub timeout: u64,

    #[serde(default = "default_retries")]
    pub retries: u32,

    #[serde(default = "default_retry_delay")]
    pub retry_delay: u64,

    #[serde(default = "default_max_delay")]
    pub max_delay: u64,

    #[serde(default = "default_backoff")]
    pub backoff: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageConfig {
    #[serde(default = "default_scale")]
    pub scale: f32,

    #[serde(default = "default_format")]
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PerformanceConfig {
    #[serde(default = "default_concurrent")]
    pub concurrent: usize,

    #[serde(default = "default_chunk_size")]
    pub chunk_size: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    #[serde(default = "default_ttl")]
    pub ttl: u64,

    pub path: Option<PathBuf>,
}

// Alias for backward compatibility
pub use CacheConfig as Cache;
pub use ExtractionConfig as Extraction;
pub use HttpConfig as Http;
pub use ImageConfig as Images;
pub use PerformanceConfig as Performance;

// Default values
const fn default_depth() -> u32 {
    5
}
const fn default_max_depth() -> u32 {
    10
}
const fn default_include_styles() -> bool {
    true
}
const fn default_include_components() -> bool {
    true
}
const fn default_include_vectors() -> bool {
    false
}
const fn default_timeout() -> u64 {
    30
}
const fn default_retries() -> u32 {
    3
}
const fn default_retry_delay() -> u64 {
    1000
}
const fn default_max_delay() -> u64 {
    60000
}
const fn default_backoff() -> f64 {
    2.0
}
const fn default_scale() -> f32 {
    2.0
}
fn default_format() -> String {
    "png".to_string()
}
const fn default_concurrent() -> usize {
    50
}
const fn default_chunk_size() -> usize {
    100
}
const fn default_ttl() -> u64 {
    24
}

// Default implementations
impl Default for ExtractionConfig {
    fn default() -> Self {
        Self {
            depth: default_depth(),
            max_depth: default_max_depth(),
            styles: default_include_styles(),
            components: default_include_components(),
            vectors: default_include_vectors(),
        }
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            timeout: default_timeout(),
            retries: default_retries(),
            retry_delay: default_retry_delay(),
            max_delay: default_max_delay(),
            backoff: default_backoff(),
        }
    }
}

impl Default for ImageConfig {
    fn default() -> Self {
        Self {
            scale: default_scale(),
            format: default_format(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            concurrent: default_concurrent(),
            chunk_size: default_chunk_size(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl: default_ttl(),
            path: None,
        }
    }
}

impl Config {
    /// Load config with default paths
    pub fn load() -> Result<Self> {
        Self::load_with(None, None, None)
    }

    /// Load config from specific path
    pub fn load_from(path: impl AsRef<Path>) -> Result<Self> {
        Self::load_with(Some(path.as_ref().to_path_buf()), None, None)
    }

    /// Load config with all options
    pub fn load_with(
        config_path: Option<PathBuf>,
        cli_token: Option<String>,
        cli_cache_dir: Option<PathBuf>,
    ) -> Result<Self> {
        let mut config = Self::default();

        // Load from file (check project config first, then global)
        let path = if let Some(p) = config_path {
            Some(p)
        } else {
            // Check project config first
            let project_config = PathBuf::from("figma-cli.toml");
            if project_config.exists() {
                Some(project_config)
            } else {
                Self::default_config_path()
            }
        };

        if let Some(p) = path.filter(|p| p.exists()) {
            let content = std::fs::read_to_string(&p)?;
            config = toml::from_str(&content)?;
        }

        // Override with environment
        if let Ok(token) = std::env::var("FIGMA_TOKEN") {
            config.token = Some(token);
        }

        // Override with CLI args
        if let Some(token) = cli_token {
            config.token = Some(token);
        }
        if let Some(dir) = cli_cache_dir {
            config.cache.path = Some(dir);
        }

        config.validate()?;
        Ok(config)
    }

    /// Save config to file
    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get default config path
    /// Uses XDG_CONFIG_HOME or ~/.config on all platforms
    pub fn default_config_path() -> Option<PathBuf> {
        std::env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|mut p| {
                p.push(".config");
                p
            }))
            .map(|mut p| {
                p.push("figma-cli");
                p.push("config.toml");
                p
            })
    }

    /// Get config path (for compatibility)
    pub fn config_path() -> Option<PathBuf> {
        Self::default_config_path()
    }

    /// Get default cache directory
    pub fn default_cache_dir() -> Option<PathBuf> {
        dirs::cache_dir().map(|mut p| {
            p.push("figma-cli");
            p
        })
    }

    /// Get cache directory (for compatibility)
    pub fn cache_dir() -> Option<PathBuf> {
        Self::default_cache_dir()
    }

    /// Get cache path
    pub fn cache_path(&self) -> PathBuf {
        self.cache
            .path
            .clone()
            .or_else(Self::default_cache_dir)
            .unwrap_or_else(|| PathBuf::from("/tmp/figma-cli"))
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.extraction.depth > self.extraction.max_depth {
            anyhow::bail!("depth exceeds max_depth");
        }

        if self.http.timeout == 0 {
            anyhow::bail!("timeout must be > 0");
        }

        if self.images.scale <= 0.0 || self.images.scale > 4.0 {
            anyhow::bail!("scale must be 0 < scale <= 4");
        }

        if !["png", "jpg", "svg", "pdf"].contains(&self.images.format.as_str()) {
            anyhow::bail!("invalid format: {}", self.images.format);
        }

        Ok(())
    }

    /// Display configuration (optionally as JSON)
    pub fn show(&self, json: bool) -> Result<()> {
        let mut masked = self.clone();
        if let Some(token) = &masked.token {
            masked.token = Some(mask_token(token));
        }

        if json {
            println!("{}", serde_json::to_string_pretty(&masked)?);
        } else {
            println!("token: {}", masked.token.as_deref().unwrap_or("-"));
            println!("extraction.depth: {}", masked.extraction.depth);
            println!("http.timeout: {}s", masked.http.timeout);
            println!("images.scale: {}x", masked.images.scale);
            println!("images.format: {}", masked.images.format);
        }

        Ok(())
    }

    /// Initialize default config file
    pub fn init() -> Result<()> {
        let path = Self::default_config_path().context("no config path")?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        if path.exists() {
            anyhow::bail!("config exists: {}", path.display());
        }

        let config = Self::default();
        config.save(&path)?;

        println!("created: {}", path.display());
        Ok(())
    }

    // Compatibility accessors
    pub const fn extract(&self) -> &ExtractionConfig {
        &self.extraction
    }
}

fn mask_token(token: &str) -> String {
    if token.len() <= 8 {
        "*".repeat(token.len())
    } else {
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    }
}

/// Secure string that zeros memory on drop
#[derive(Debug, Clone)]
pub struct SecureString(String);

impl SecureString {
    pub const fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl Serialize for SecureString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SecureString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(Self::new)
    }
}
