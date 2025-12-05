use crate::core::errors::Error;
use blake3::Hasher;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub file_key: String,
    pub version: String,
    pub data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub depth: Option<u32>,
}

#[derive(Debug)]
pub struct Cache {
    dir: PathBuf,
    ttl_hours: u64,
    index: Arc<RwLock<HashMap<String, CacheMetadata>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub cache_key: String,
    pub file_key: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub accessed_at: DateTime<Utc>,
    pub depth: Option<u32>,
    pub size: u64,
}

impl Cache {
    pub fn new(dir: PathBuf, ttl_hours: u64) -> Result<Self> {
        std::fs::create_dir_all(&dir)
            .map_err(|e| Error::other(format!("Failed to create cache dir: {e}")))?;

        let mut cache = Self {
            dir,
            ttl_hours,
            index: Arc::new(RwLock::new(HashMap::new())),
        };

        cache.load_index()?;
        cache.cleanup_expired()?;

        Ok(cache)
    }

    pub fn get_file(
        &self,
        file_key: &str,
        depth: Option<u32>,
    ) -> Result<Option<serde_json::Value>> {
        let cache_key = self.cache_key(file_key, depth);
        let path = self.entry_path(&cache_key);

        if !path.exists() {
            return Ok(None);
        }

        let entry: CacheEntry = self.read_entry(&path)?;

        if self.is_expired(&entry) {
            self.remove_entry(&cache_key)?;
            return Ok(None);
        }

        self.update_access_time(&cache_key)?;
        Ok(Some(entry.data))
    }

    pub fn put_file(
        &self,
        file_key: &str,
        version: &str,
        data: &serde_json::Value,
        depth: Option<u32>,
    ) -> Result<()> {
        let cache_key = self.cache_key(file_key, depth);
        let entry = CacheEntry {
            file_key: file_key.to_string(),
            version: version.to_string(),
            data: data.clone(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            depth,
        };

        self.write_entry(&cache_key, &entry)?;
        Ok(())
    }

    pub fn get_nodes(
        &self,
        file_key: &str,
        node_ids: &[String],
        depth: Option<u32>,
    ) -> Result<Option<serde_json::Value>> {
        let cache_key = self.nodes_cache_key(file_key, node_ids, depth);
        let path = self.entry_path(&cache_key);

        if !path.exists() {
            return Ok(None);
        }

        let entry: CacheEntry = self.read_entry(&path)?;

        if self.is_expired(&entry) {
            self.remove_entry(&cache_key)?;
            return Ok(None);
        }

        self.update_access_time(&cache_key)?;
        Ok(Some(entry.data))
    }

    pub fn put_nodes(
        &self,
        file_key: &str,
        node_ids: &[String],
        depth: Option<u32>,
        data: &serde_json::Value,
    ) -> Result<()> {
        let cache_key = self.nodes_cache_key(file_key, node_ids, depth);
        let entry = CacheEntry {
            file_key: file_key.to_string(),
            version: String::new(),
            data: data.clone(),
            created_at: Utc::now(),
            accessed_at: Utc::now(),
            depth,
        };

        self.write_entry(&cache_key, &entry)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        let index = self.index.read();
        for meta in index.values() {
            let path = self.entry_path(&meta.cache_key);
            let _ = std::fs::remove_file(path);
        }
        drop(index);

        self.index.write().clear();
        self.save_index()?;
        Ok(())
    }

    pub fn list(&self) -> Vec<CacheMetadata> {
        self.index.read().values().cloned().collect()
    }

    pub fn stats(&self) -> CacheStats {
        let index = self.index.read();
        let total_entries = index.len();
        let total_size: u64 = index.values().map(|m| m.size).sum();

        let now = Utc::now();
        let expired = index
            .values()
            .filter(|m| {
                let age = now.signed_duration_since(m.created_at);
                age.num_hours() > self.ttl_hours as i64
            })
            .count();

        CacheStats {
            total_entries,
            total_size,
            expired_entries: expired,
            ttl_hours: self.ttl_hours,
        }
    }

    fn cache_key(&self, file_key: &str, depth: Option<u32>) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"file:");
        hasher.update(file_key.as_bytes());
        if let Some(d) = depth {
            hasher.update(b":depth:");
            hasher.update(d.to_string().as_bytes());
        }
        hasher.finalize().to_hex().to_string()
    }

    fn nodes_cache_key(&self, file_key: &str, node_ids: &[String], depth: Option<u32>) -> String {
        let mut hasher = Hasher::new();
        hasher.update(b"nodes:");
        hasher.update(file_key.as_bytes());
        for id in node_ids {
            hasher.update(b":");
            hasher.update(id.as_bytes());
        }
        if let Some(d) = depth {
            hasher.update(b":depth:");
            hasher.update(d.to_string().as_bytes());
        }
        hasher.finalize().to_hex().to_string()
    }

    fn entry_path(&self, cache_key: &str) -> PathBuf {
        self.dir.join(format!("{cache_key}.json"))
    }

    fn index_path(&self) -> PathBuf {
        self.dir.join("index.json")
    }

    fn read_entry(&self, path: &Path) -> Result<CacheEntry> {
        let data = std::fs::read(path)
            .map_err(|e| Error::other(format!("Failed to read cache entry: {e}")))?;
        serde_json::from_slice(&data)
            .map_err(|e| Error::parse(format!("Failed to parse cache entry: {e}")))
    }

    fn write_entry(&self, cache_key: &str, entry: &CacheEntry) -> Result<()> {
        let path = self.entry_path(cache_key);
        let data = serde_json::to_vec(entry)
            .map_err(|e| Error::other(format!("Failed to serialize cache entry: {e}")))?;

        std::fs::write(&path, &data)
            .map_err(|e| Error::other(format!("Failed to write cache entry: {e}")))?;

        let metadata = CacheMetadata {
            cache_key: cache_key.to_string(),
            file_key: entry.file_key.clone(),
            version: entry.version.clone(),
            created_at: entry.created_at,
            accessed_at: entry.accessed_at,
            depth: entry.depth,
            size: data.len() as u64,
        };

        self.index.write().insert(cache_key.to_string(), metadata);
        self.save_index()?;

        Ok(())
    }

    fn remove_entry(&self, cache_key: &str) -> Result<()> {
        let path = self.entry_path(cache_key);
        let _ = std::fs::remove_file(path);
        {
            self.index.write().remove(cache_key);
        }
        self.save_index()?;
        Ok(())
    }

    fn update_access_time(&self, cache_key: &str) -> Result<()> {
        {
            if let Some(meta) = self.index.write().get_mut(cache_key) {
                meta.accessed_at = Utc::now();
            }
        }
        self.save_index()?;
        Ok(())
    }

    fn is_expired(&self, entry: &CacheEntry) -> bool {
        let now = Utc::now();
        let age = now.signed_duration_since(entry.created_at);
        age.num_hours() > self.ttl_hours as i64
    }

    fn cleanup_expired(&self) -> Result<()> {
        let now = Utc::now();
        let expired: Vec<String> = self
            .index
            .read()
            .iter()
            .filter(|(_, meta)| {
                let age = now.signed_duration_since(meta.created_at);
                age.num_hours() > self.ttl_hours as i64
            })
            .map(|(key, _)| key.clone())
            .collect();

        for key in expired {
            self.remove_entry(&key)?;
        }

        Ok(())
    }

    fn load_index(&mut self) -> Result<()> {
        let path = self.index_path();
        if !path.exists() {
            return Ok(());
        }

        let data = std::fs::read(&path)
            .map_err(|e| Error::other(format!("Failed to read cache index: {e}")))?;

        let index: HashMap<String, CacheMetadata> = serde_json::from_slice(&data)
            .map_err(|e| Error::parse(format!("Failed to parse cache index: {e}")))?;

        *self.index.write() = index;
        Ok(())
    }

    fn save_index(&self) -> Result<()> {
        let path = self.index_path();
        let data = serde_json::to_vec(&*self.index.read())
            .map_err(|e| Error::other(format!("Failed to serialize cache index: {e}")))?;

        std::fs::write(&path, data)
            .map_err(|e| Error::other(format!("Failed to write cache index: {e}")))?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size: u64,
    pub expired_entries: usize,
    pub ttl_hours: u64,
}
