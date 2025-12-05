# figma-cli - AI Agent Developer Guide

Essential implementation knowledge for this Rust Figma CLI.

---

## Architecture

```
src/
├── core/              # Domain logic (zero dependencies on outer layers)
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
│   ├── context.rs     # ClientContext (config/token/cache/client setup)
│   └── args.rs        # CLI arguments (clap)
├── extractor/         # Content extractors
│   └── text.rs        # Text node extraction visitor
└── models/            # Data structures
    └── document.rs    # Figma API response types
```

**Key Design**: Hexagonal architecture - `core/` has zero dependencies on outer layers. All I/O happens through `client/` and `cli/` adapters.

---

## Critical Implementation Patterns

### 1. Node Composition Pattern

**Why**: Eliminated massive code duplication (22 variants × 5 common fields)

**Location**: `src/models/document.rs`

```rust
// Common fields extracted to NodeBase
pub struct NodeBase {
    #[serde(rename = "type")]
    pub node_type: String,
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
}

// Main Node struct with composition
pub struct Node {
    pub base: NodeBase,
    pub data: NodeData,
}

// Variant-specific data
#[serde(untagged)]
pub enum NodeData {
    Canvas { background_color: Option<Color>, children: Vec<Node>, ... },
    Frame { absolute_bounding_box: Option<BoundingBox>, children: Vec<Node>, ... },
    Text { characters: String, style: Option<TypeStyle>, ... },
    // ... other variants
}

// Accessor methods
impl Node {
    pub fn id(&self) -> &str { &self.base.id }
    pub fn name(&self) -> &str { &self.base.name }
    pub fn children(&self) -> Option<&[Node]> { ... }
}
```

**Critical**: Custom `Serialize` and `Deserialize` implementations flatten the structure for API compatibility.

### 2. Cache System

**Files**: `src/core/cache.rs`, `src/client/figma.rs`

**Design**:
- Hash: `BLAKE3(file_key + depth)` for cache keys
- Index: `Arc<RwLock<HashMap<String, CacheMetadata>>>` for O(1) lookups
- Storage: JSON files in `~/Library/Caches/figma-cli/`
- TTL: 24h default (configurable)
- Interface: Pure `serde_json::Value` (no domain model dependencies)

**Auto-caching integration**:
```rust
// In FigmaClient::get_file()
if let Some(cache) = &self.cache
    && let Ok(Some(cached_value)) = cache.get_file(key, depth) {
    // Deserialize in client layer
    let file: FigmaFile = serde_json::from_value(cached_value)?;
    return Ok(file);
}
let file = self.http_get(...).await?;
if let Some(cache) = &self.cache {
    // Serialize in client layer
    let value = serde_json::to_value(&file)?;
    cache.put_file(key, version, &value, depth)?;
}
Ok(file)
```

### 3. Query Engine (JMESPath)

**Location**: `src/core/query.rs`

**Critical**: Use `serde_json::to_value(n)` for Number conversion
```rust
jmespath::Variable::Number(n) => serde_json::to_value(n)
    .map_err(|e| Error::other(format!("Failed to convert: {e}"))),
```

**Why**: JMESPath's `Number` type doesn't directly map to `serde_json::Number`.

### 4. Visitor Pattern for Traversal

**Location**: `src/service/traversal.rs`

**Why**: Decouples traversal from extraction logic. Add new extractors without modifying traversal.

```rust
pub trait NodeVisitor {
    fn visit_node(&mut self, node: &Node, depth: usize, path: &[String]);
}

fn traverse_node<V: NodeVisitor>(
    node: &Node, visitor: &mut V, depth: usize, path: &mut Vec<String>
) {
    visitor.visit_node(node, depth, path);
    if let Some(children) = node.children()
        && !children.is_empty()
    {
        path.push(node.name().to_string());
        for child in children {
            traverse_node(child, visitor, depth + 1, path);
        }
        path.pop();
    }
}
```

### 5. ClientContext for DRY Command Handlers

**Location**: `src/cli/context.rs`

**Why**: Eliminates repeated boilerplate in command handlers.

```rust
pub struct ClientContext {
    pub config: Config,
    pub client: FigmaClient,
    pub token: String,
}

impl ClientContext {
    pub fn new(config_path: Option<&str>) -> Result<Self> {
        // Loads config, gets token, sets up cache, creates client
    }
}

// Usage in command handlers:
let ctx = ClientContext::new(args.config.as_deref())?;
let result = ctx.client.get_file(&file_key, depth).await?;
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
3. Use `traverse_document()` with your visitor

---

## File Locations

- **Config**: `~/.config/figma-cli/config.toml`
- **Cache**: `~/Library/Caches/figma-cli/` (macOS), `~/.cache/figma-cli/` (Linux)
- **Binary**: `~/.local/bin/figma-cli` or `~/.cargo/bin/figma-cli`

---

## Testing

```bash
cargo test                    # All tests (45 total)
cargo test cache::            # Cache module
cargo test query::            # Query module
FIGMA_TOKEN=xxx cargo run -- extract FILE_KEY
```

---

End of guide. See [README.md](README.md) for user documentation.
