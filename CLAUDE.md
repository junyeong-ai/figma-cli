# figma-cli - AI Agent Developer Guide

Essential implementation knowledge for this Rust Figma CLI.

---

## Architecture

```
src/
├── core/              # Domain logic (zero dependencies)
│   ├── cache.rs       # BLAKE3-hashed cache (RwLock + HashMap)
│   ├── query.rs       # JMESPath query engine
│   ├── config.rs      # Hierarchical config system
│   └── errors.rs      # Error types
├── client/            # HTTP adapter
│   ├── figma.rs       # API client with auto-caching
│   └── retry.rs       # Exponential backoff
├── service/           # Business logic
│   ├── orchestrator.rs  # Extraction coordinator
│   └── traversal.rs     # Visitor pattern tree traversal
├── cli/               # User interface
│   ├── commands.rs    # Command handlers
│   └── args.rs        # CLI arguments (clap)
└── models/            # Data structures
    └── document.rs    # Figma API response types
```

**Key Design**: Hexagonal architecture - `core/` has zero dependencies on outer layers. All I/O happens through `client/` and `cli/` adapters.

---

## Critical Implementation Patterns

### 1. Serde Untagged Enum for Figma Nodes

**Why**: Figma's `Option<Color>`, `Option<BoundingBox>` fields fail with internally tagged enums

**Location**: `src/models/document.rs:53-618`

```rust
#[serde(untagged)]
pub enum Node {
    Canvas {
        #[serde(with = "option_struct")]
        background_color: Option<Color>,
        // ...
    },
}

mod option_struct {
    pub fn deserialize<'de, D, T>(d: D) -> Result<Option<T>, D::Error> {
        let v = Value::deserialize(d)?;
        Ok(if v.is_null() { None } else { Some(from_value(v)?) })
    }
}
```

**Critical**: Don't change to `#[serde(tag = "type")]` - it breaks `Option<Struct>` deserialization.

### 2. Cache System

**Files**: `src/core/cache.rs`, `src/client/figma.rs:99-120`

**Design**:
- Hash: `BLAKE3(file_key + depth)` for cache keys
- Index: `Arc<RwLock<HashMap<String, CacheMetadata>>>` for O(1) lookups
- Storage: JSON files in `~/Library/Caches/figma-cli/`
- TTL: 24h default (configurable)
- Separation: File cache vs node cache (different endpoints)

**Auto-caching integration**:
```rust
// In FigmaClient::get_file()
if let Some(cache) = &self.cache
    && let Ok(Some(cached)) = cache.get_file(key, depth) {
    return Ok(cached);  // Cache hit
}
let file = self.http_get(...).await?;  // API call
if let Some(cache) = &self.cache {
    cache.put_file(&file, depth)?;  // Populate
}
Ok(file)
```

**Critical Lock Pattern** (`src/core/cache.rs:245-263`):
```rust
fn update_access_time(&self, key: &str) -> Result<()> {
    {
        // Acquire write lock in block scope
        if let Some(meta) = self.index.write().get_mut(key) {
            meta.accessed_at = Utc::now();
        }
    } // Lock released here
    self.save_index()?;  // Now safe to acquire read lock
    Ok(())
}
```

**Why**: Calling `save_index()` while holding write lock → deadlock (read lock attempt while write-locked).

### 3. Query Engine (JMESPath)

**Location**: `src/core/query.rs`

**Critical**: Use `serde_json::to_value(n)` for Number conversion
```rust
jmespath::Variable::Number(n) => serde_json::to_value(n)
    .map_err(|e| Error::other(format!("Failed to convert: {e}"))),
```

**Why**: JMESPath's `Number` type doesn't directly map to `serde_json::Number`.

### 4. Visitor Pattern for Traversal

**Location**: `src/service/traversal.rs:19-43`

**Why**: Decouples traversal from extraction logic. Add new extractors without modifying traversal.

```rust
pub trait NodeVisitor {
    fn visit_node(&mut self, node: &Node, depth: usize, path: &[String]);
}

pub fn traverse_node<V: NodeVisitor>(
    node: &Node, visitor: &mut V, depth: usize, path: &mut Vec<String>
) {
    visitor.visit_node(node, depth, path);
    if !get_children(node).is_empty() {
        path.push(get_node_name(node).to_string());
        for child in get_children(node) {
            traverse_node(child, visitor, depth + 1, path);
        }
        path.pop();
    }
}
```

---

## Common Development Tasks

### Add New Command

1. `src/cli/args.rs`: Add variant to `Commands` enum + args struct
2. `src/cli/commands.rs`: Implement `pub async fn handle_*()` function
3. `src/cli/mod.rs`: Add to `pub use commands::{...};`
4. `src/main.rs`: Add match arm in `main()`

### Add New API Endpoint

1. `src/models/document.rs`: Define response types
2. `src/client/figma.rs`: Add method to `FigmaClient`
3. Integrate caching: check cache → API call → populate cache

### Add New Visitor (Extractor)

1. Create struct in `src/extractor/`
2. Implement `NodeVisitor` trait
3. Use `traverse_node()` with your visitor

---

## File Locations

- **Config**: `~/.config/figma-cli/config.toml`
- **Cache**: `~/Library/Caches/figma-cli/` (macOS), `~/.cache/figma-cli/` (Linux)
- **Binary**: `~/.local/bin/figma-cli` or `~/.cargo/bin/figma-cli`

---

## Testing

```bash
cargo test                    # All tests (42 total)
cargo test cache::            # Cache module
cargo test query::            # Query module
FIGMA_TOKEN=xxx cargo run -- extract FILE_KEY
```

---

End of guide. See [README.md](README.md) for user documentation.
