use crate::core::config;
use crate::core::errors::{Error, Result};

pub struct TokenManager;

impl TokenManager {
    pub fn store(token: &str) -> Result<()> {
        validate_token(token)?;

        let path = config::Config::config_path()
            .ok_or_else(|| Error::io("cannot determine config path"))?;

        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut config = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            toml::from_str(&content)?
        } else {
            config::Config::default()
        };

        config.token = Some(token.to_string());
        let content = toml::to_string_pretty(&config)?;
        std::fs::write(&path, content)?;

        Ok(())
    }

    pub fn get() -> Result<Option<String>> {
        if let Ok(token) = std::env::var("FIGMA_TOKEN") {
            return Ok(Some(token));
        }

        let path = config::Config::config_path();
        if let Some(p) = path.filter(|p| p.exists()) {
            let content = std::fs::read_to_string(&p)?;
            let config: config::Config = toml::from_str(&content)?;
            return Ok(config.token);
        }

        Ok(None)
    }

    pub fn delete() -> Result<()> {
        let path = config::Config::config_path()
            .ok_or_else(|| Error::io("cannot determine config path"))?;

        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let mut config: config::Config = toml::from_str(&content)?;
            config.token = None;
            let content = toml::to_string_pretty(&config)?;
            std::fs::write(&path, content)?;
        }

        Ok(())
    }
}

fn validate_token(token: &str) -> Result<()> {
    if token.len() < 10 {
        return Err(Error::validation("token", "too short"));
    }

    if token.len() > 256 {
        return Err(Error::validation("token", "too long"));
    }

    if !token.starts_with("figd_") {
        return Err(Error::validation("token", "must start with 'figd_'"));
    }

    Ok(())
}
