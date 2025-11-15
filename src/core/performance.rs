//! High-performance caching and parallel processing
//!
//! This module implements:
//! - Multi-layer caching (L1 memory, L2 disk)
//! - Lock-free concurrent data structures
//! - Work-stealing parallel processing
//! - SIMD optimizations for JSON parsing

use bytes::Bytes;
use dashmap::DashMap;
use lru::LruCache;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::{Duration, Instant};
// Cache Types
/// Multi-layer cache system
pub struct MultiLayerCache {
    l1_memory: Arc<L1Cache>,
    l2_disk: Option<Arc<L2Cache>>,
    stats: Arc<CacheStats>,
}

impl MultiLayerCache {
    /// Create a new multi-layer cache
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1_memory: Arc::new(L1Cache::new(config.l1_size)),
            l2_disk: config
                .l2_enabled
                .then(|| Arc::new(L2Cache::new(config.l2_path))),
            stats: Arc::new(CacheStats::default()),
        }
    }

    /// Get value from cache
    pub async fn get(&self, key: &str) -> Option<CachedValue> {
        // Try L1 first
        if let Some(value) = self.l1_memory.get(key) {
            self.stats.record_hit(CacheLevel::L1);
            return Some(value);
        }

        // Try L2 if enabled
        if let Some(l2) = &self.l2_disk
            && let Some(value) = l2.get(key).await
        {
            self.stats.record_hit(CacheLevel::L2);
            // Promote to L1
            self.l1_memory.put(key.to_string(), value.clone());
            return Some(value);
        }

        self.stats.record_miss();
        None
    }

    /// Put value into cache
    pub async fn put(&self, key: String, value: CachedValue) {
        // Always put in L1
        self.l1_memory.put(key.clone(), value.clone());

        // Optionally put in L2
        if let Some(l2) = &self.l2_disk {
            l2.put(key, value).await;
        }
    }

    /// Invalidate cache entry
    pub async fn invalidate(&self, key: &str) {
        self.l1_memory.remove(key);
        if let Some(l2) = &self.l2_disk {
            l2.remove(key).await;
        }
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStatsSummary {
        self.stats.summary()
    }
}

/// L1 in-memory cache (hot data)
struct L1Cache {
    data: Arc<DashMap<String, CacheEntry>>,
    lru: Arc<Mutex<LruCache<String, ()>>>,
}

impl L1Cache {
    fn new(capacity: usize) -> Self {
        Self {
            data: Arc::new(DashMap::new()),
            lru: Arc::new(Mutex::new(LruCache::new(
                NonZeroUsize::new(capacity).unwrap(),
            ))),
        }
    }

    fn get(&self, key: &str) -> Option<CachedValue> {
        let entry = self.data.get(key)?;

        // Check TTL
        if entry.is_expired() {
            self.remove(key);
            return None;
        }

        // Update LRU
        self.lru.lock().get(key);

        Some(entry.value.clone())
    }

    fn put(&self, key: String, value: CachedValue) {
        let entry = CacheEntry::new(value, Duration::from_secs(3600));

        // Update LRU and handle eviction
        let mut lru = self.lru.lock();
        if lru.len() >= lru.cap().get()
            && let Some((evicted_key, ())) = lru.pop_lru()
        {
            self.data.remove(&evicted_key);
        }
        lru.put(key.clone(), ());
        drop(lru);

        self.data.insert(key, entry);
    }

    fn remove(&self, key: &str) {
        self.data.remove(key);
        self.lru.lock().pop(key);
    }
}

/// L2 disk cache (cold data)
struct L2Cache {
    path: std::path::PathBuf,
}

impl L2Cache {
    fn new(path: std::path::PathBuf) -> Self {
        // Create cache directory if it doesn't exist
        if let Err(e) = std::fs::create_dir_all(&path) {
            tracing::warn!("Failed to create L2 cache directory: {}", e);
        }
        Self { path }
    }

    async fn get(&self, key: &str) -> Option<CachedValue> {
        let file_path = self.key_to_path(key);
        let data = tokio::fs::read(&file_path).await.ok()?;

        // Simple format: content_type_len (4 bytes) | content_type | metadata_len (4 bytes) | metadata_json | data
        if data.len() < 8 {
            return None;
        }

        let content_type_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        if data.len() < 4 + content_type_len + 4 {
            return None;
        }

        let content_type = String::from_utf8(data[4..4 + content_type_len].to_vec()).ok()?;

        let metadata_len_start = 4 + content_type_len;
        let metadata_len = u32::from_le_bytes([
            data[metadata_len_start],
            data[metadata_len_start + 1],
            data[metadata_len_start + 2],
            data[metadata_len_start + 3],
        ]) as usize;

        let metadata_start = metadata_len_start + 4;
        let metadata: HashMap<String, String> =
            serde_json::from_slice(&data[metadata_start..metadata_start + metadata_len]).ok()?;

        let data_start = metadata_start + metadata_len;
        let cached_data = Bytes::copy_from_slice(&data[data_start..]);

        Some(CachedValue {
            data: cached_data,
            content_type,
            metadata,
        })
    }

    async fn put(&self, key: String, value: CachedValue) {
        let file_path = self.key_to_path(&key);

        // Create parent directories
        if let Some(parent) = file_path.parent() {
            let _ = tokio::fs::create_dir_all(parent).await;
        }

        // Serialize value
        let content_type_bytes = value.content_type.as_bytes();
        let metadata_bytes = serde_json::to_vec(&value.metadata).unwrap_or_default();

        let content_type_len = (content_type_bytes.len() as u32).to_le_bytes();
        let metadata_len = (metadata_bytes.len() as u32).to_le_bytes();

        let mut buffer = Vec::with_capacity(
            4 + content_type_bytes.len() + 4 + metadata_bytes.len() + value.data.len(),
        );
        buffer.extend_from_slice(&content_type_len);
        buffer.extend_from_slice(content_type_bytes);
        buffer.extend_from_slice(&metadata_len);
        buffer.extend_from_slice(&metadata_bytes);
        buffer.extend_from_slice(&value.data);

        let _ = tokio::fs::write(&file_path, buffer).await;
    }

    async fn remove(&self, key: &str) {
        let file_path = self.key_to_path(key);
        let _ = tokio::fs::remove_file(&file_path).await;
    }

    fn key_to_path(&self, key: &str) -> std::path::PathBuf {
        // Hash the key to create a safe filename
        let hash = blake3::hash(key.as_bytes());
        let hex = hash.to_hex();

        // Use first 2 chars as subdirectory for better distribution
        let subdir = &hex[0..2];
        self.path.join(subdir).join(&hex[2..])
    }
}

/// Cached value wrapper
#[derive(Clone, Debug)]
pub struct CachedValue {
    pub data: Bytes,
    pub content_type: String,
    pub metadata: HashMap<String, String>,
}

/// Cache entry with TTL
struct CacheEntry {
    value: CachedValue,
    expires_at: Instant,
}

impl CacheEntry {
    fn new(value: CachedValue, ttl: Duration) -> Self {
        Self {
            value,
            expires_at: Instant::now() + ttl,
        }
    }

    fn is_expired(&self) -> bool {
        Instant::now() > self.expires_at
    }
}

/// Cache configuration
#[derive(Clone)]
pub struct CacheConfig {
    pub l1_size: usize,
    pub l2_enabled: bool,
    pub l2_path: std::path::PathBuf,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            l1_size: 1000,
            l2_enabled: false,
            l2_path: std::path::PathBuf::from("/tmp/figma-cache"),
        }
    }
}

/// Cache statistics
#[derive(Default)]
struct CacheStats {
    l1_hits: std::sync::atomic::AtomicU64,
    l2_hits: std::sync::atomic::AtomicU64,
    misses: std::sync::atomic::AtomicU64,
}

impl CacheStats {
    fn record_hit(&self, level: CacheLevel) {
        use std::sync::atomic::Ordering;
        match level {
            CacheLevel::L1 => self.l1_hits.fetch_add(1, Ordering::Relaxed),
            CacheLevel::L2 => self.l2_hits.fetch_add(1, Ordering::Relaxed),
        };
    }

    fn record_miss(&self) {
        use std::sync::atomic::Ordering;
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    fn summary(&self) -> CacheStatsSummary {
        use std::sync::atomic::Ordering;
        let l1_hits = self.l1_hits.load(Ordering::Relaxed);
        let l2_hits = self.l2_hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = l1_hits + l2_hits + misses;

        CacheStatsSummary {
            l1_hits,
            l2_hits,
            misses,
            hit_rate: if total > 0 {
                ((l1_hits + l2_hits) as f64 / total as f64) * 100.0
            } else {
                0.0
            },
        }
    }
}

#[derive(Clone, Copy)]
enum CacheLevel {
    L1,
    L2,
}

/// Cache statistics summary
#[derive(Debug, Clone)]
pub struct CacheStatsSummary {
    pub l1_hits: u64,
    pub l2_hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
}
// Content-Addressable Cache
/// Content-addressable cache using blake3 hashing
pub struct ContentCache {
    cache: Arc<DashMap<[u8; 32], Bytes>>,
}

impl Default for ContentCache {
    fn default() -> Self {
        Self::new()
    }
}

impl ContentCache {
    /// Create a new content cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Store content and return its hash
    pub fn store(&self, content: &[u8]) -> [u8; 32] {
        let hash = blake3::hash(content);
        let hash_bytes = *hash.as_bytes();
        self.cache
            .insert(hash_bytes, Bytes::copy_from_slice(content));
        hash_bytes
    }

    /// Retrieve content by hash
    pub fn get(&self, hash: &[u8; 32]) -> Option<Bytes> {
        self.cache.get(hash).map(|v| v.clone())
    }

    /// Check if content exists
    pub fn contains(&self, hash: &[u8; 32]) -> bool {
        self.cache.contains_key(hash)
    }
}
// Parallel Processing
use crossbeam::channel::{Receiver, Sender, bounded};
use rayon::prelude::*;

/// Parallel processor for batch operations
pub struct ParallelProcessor {
    thread_pool: rayon::ThreadPool,
}

impl ParallelProcessor {
    /// Create a new parallel processor
    pub fn new(num_threads: usize) -> Self {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("Failed to create thread pool");

        Self { thread_pool }
    }

    /// Process items in parallel with a given function
    pub fn process<T, R, F>(&self, items: Vec<T>, processor: F) -> Vec<R>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&T) -> R + Send + Sync,
    {
        self.thread_pool
            .install(|| items.par_iter().map(processor).collect())
    }

    /// Process items with backpressure control
    pub fn process_with_backpressure<T, R, F>(
        &self,
        items: Vec<T>,
        processor: F,
        buffer_size: usize,
    ) -> Vec<R>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: Fn(T) -> R + Send + Sync + Clone + 'static,
    {
        let (tx, rx): (Sender<R>, Receiver<R>) = bounded(buffer_size);
        let items_len = items.len();

        // Process items in parallel with bounded channel for backpressure
        self.thread_pool.scope(|s| {
            for item in items {
                let tx = tx.clone();
                let processor = processor.clone();
                s.spawn(move |_| {
                    let result = processor(item);
                    let _ = tx.send(result);
                });
            }
        });

        // Collect results
        let mut results = Vec::with_capacity(items_len);
        for _ in 0..items_len {
            results.push(
                rx.recv()
                    .expect("all senders are alive in scope, recv should not fail"),
            );
        }
        results
    }

    /// Map-reduce operation
    pub fn map_reduce<T, M, F, G>(&self, items: Vec<T>, map: F, reduce: G, initial: M) -> M
    where
        T: Send + Sync,
        M: Send + Sync + Clone,
        F: Fn(&T) -> M + Send + Sync,
        G: Fn(M, M) -> M + Send + Sync,
    {
        self.thread_pool
            .install(|| items.par_iter().map(map).reduce(|| initial.clone(), reduce))
    }
}
// SIMD JSON Parsing (placeholder)
/// SIMD-accelerated JSON parser
pub struct SimdJsonParser;

impl SimdJsonParser {
    /// Parse JSON using SIMD instructions
    pub fn parse(json: &[u8]) -> crate::core::Result<serde_json::Value> {
        // This is a placeholder - real implementation would use simdjson-rs
        serde_json::from_slice(json).map_err(|e| crate::core::errors::Error::parse(e.to_string()))
    }

    /// Validate JSON using SIMD
    pub fn validate(json: &[u8]) -> bool {
        // Placeholder - real implementation would use SIMD validation
        serde_json::from_slice::<serde_json::Value>(json).is_ok()
    }
}
// Work Stealing Queue
use crossbeam::deque::{Injector, Stealer, Worker};

/// Work-stealing queue for load balancing
pub struct WorkStealingQueue<T> {
    injector: Arc<Injector<T>>,
    workers: Vec<Worker<T>>,
    stealers: Vec<Stealer<T>>,
}

impl<T> WorkStealingQueue<T> {
    /// Create a new work-stealing queue
    pub fn new(num_workers: usize) -> Self {
        let injector = Arc::new(Injector::new());
        let mut workers = Vec::with_capacity(num_workers);
        let mut stealers = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let worker = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        Self {
            injector,
            workers,
            stealers,
        }
    }

    /// Push work to the global queue
    pub fn push(&self, task: T) {
        self.injector.push(task);
    }

    /// Try to pop work for a specific worker
    pub fn pop(&self, worker_id: usize) -> Option<T> {
        // Try local queue first
        if let Some(task) = self.workers[worker_id].pop() {
            return Some(task);
        }

        // Try stealing from global injector
        loop {
            match self.injector.steal_batch_and_pop(&self.workers[worker_id]) {
                crossbeam::deque::Steal::Success(task) => return Some(task),
                crossbeam::deque::Steal::Empty => break,
                crossbeam::deque::Steal::Retry => {}
            }
        }

        // Try stealing from other workers
        for (i, stealer) in self.stealers.iter().enumerate() {
            if i != worker_id
                && let crossbeam::deque::Steal::Success(task) = stealer.steal()
            {
                return Some(task);
            }
        }

        None
    }
}
// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multi_layer_cache() {
        let config = CacheConfig::default();
        let cache = MultiLayerCache::new(config);

        let value = CachedValue {
            data: Bytes::from("test data"),
            content_type: "text/plain".to_string(),
            metadata: HashMap::new(),
        };

        // Test put and get
        cache.put("key1".to_string(), value.clone()).await;
        let retrieved = cache.get("key1").await;
        assert!(retrieved.is_some());

        // Test cache stats
        let stats = cache.stats();
        assert_eq!(stats.l1_hits, 1);
        assert_eq!(stats.misses, 0);

        // Test invalidation
        cache.invalidate("key1").await;
        assert!(cache.get("key1").await.is_none());
    }

    #[test]
    fn test_content_cache() {
        let cache = ContentCache::new();

        let content = b"Hello, World!";
        let hash = cache.store(content);

        let retrieved = cache.get(&hash);
        assert_eq!(retrieved.unwrap(), Bytes::from(content.as_slice()));

        assert!(cache.contains(&hash));
    }

    #[test]
    fn test_parallel_processor() {
        let processor = ParallelProcessor::new(4);

        let items = vec![1, 2, 3, 4, 5];
        let results = processor.process(items, |x| x * 2);

        assert_eq!(results.len(), 5);
        assert!(results.contains(&2));
        assert!(results.contains(&4));
        assert!(results.contains(&6));
        assert!(results.contains(&8));
        assert!(results.contains(&10));
    }

    #[test]
    fn test_work_stealing_queue() {
        let queue = WorkStealingQueue::new(2);

        queue.push(1);
        queue.push(2);
        queue.push(3);

        let task1 = queue.pop(0);
        assert!(task1.is_some());

        let task2 = queue.pop(1);
        assert!(task2.is_some());
    }
}
