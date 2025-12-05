//! Client context for command handlers

use crate::client::{FigmaClient, TokenManager};
use crate::core::{Cache, Config};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;

pub struct ClientContext {
    pub config: Config,
    pub client: FigmaClient,
    pub token: String,
}

impl ClientContext {
    pub fn new(config_path: Option<&str>) -> Result<Self> {
        let config = match config_path {
            Some(path) => Config::load_from(Path::new(path))?,
            None => Config::load()?,
        };

        let token = config
            .token
            .clone()
            .or_else(|| TokenManager::get().ok().flatten())
            .context("No authentication token found. Run 'figma-cli auth login' first")?;

        let cache_dir = config.cache_path();
        let cache = Arc::new(Cache::new(cache_dir, config.cache.ttl)?);

        let client =
            FigmaClient::with_timeout(token.clone(), config.http.timeout)?.with_cache(cache);

        Ok(Self {
            config,
            client,
            token,
        })
    }
}
