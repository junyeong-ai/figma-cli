# figma-cli - AI Agent Developer Guide

Essential knowledge for implementing features and debugging this Rust CLI tool.

---

## Core Patterns

### Untagged Enum with Generic Option<Struct> Deserializer

**Problem**: Serde's internally tagged enum (`#[serde(tag = "type")]`) cannot handle `Option<Struct>` fields due to deserialization limitations.

**Implementation**:
```rust
// Node enum with untagged pattern
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Node {
    Canvas {
        #[serde(rename = "type")]
        node_type: String,
        id: String,
        name: String,
        #[serde(
            rename = "backgroundColor",
            default,
            skip_serializing_if = "Option::is_none",
            with = "option_struct"
        )]
        background_color: Option<Color>,
        children: Vec<Node>,
    },
    // 18 more variants...
}

// Generic helper for deserializing Option<T: DeserializeOwned>
mod option_struct {
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_json::from_value(value).map_err(serde::de::Error::custom)?))
    }
}
```

**Why**: Allows complex nested Figma document structures with optional struct fields. Internally tagged enums fail with `invalid type: map, expected f64` errors when deserializing `Option<Color>`, `Option<BoundingBox>`, etc.

**Applied to**: All 19 Node variants with `Option<Color>`, `Option<BoundingBox>`, `Option<TypeStyle>` fields.

**Location**: `src/models/document.rs` lines 53-618

---

### Hierarchical Path Tracking

**Implementation**:
```rust
pub fn traverse_node<V: NodeVisitor>(
    node: &Node,
    visitor: &mut V,
    depth: usize,
    path: &mut Vec<String>,
) {
    visitor.visit_node(node, depth, path);

    let children = get_children(node);
    if !children.is_empty() {
        path.push(get_node_name(node).to_string());
        for child in children {
            traverse_node(child, visitor, depth + 1, path);
        }
        path.pop();
    }
}
```

**Why**: Maintains hierarchical context during tree traversal. Path structure: `[Document, Canvas, Frame, ...]` enables text extraction with full location context (`Page > Frame > Group > Text`).

**Pattern**: Pass `&mut Vec<String>` through recursion, pushing node names before descending, popping after visiting all children.

**Location**: `src/service/traversal.rs` lines 19-43

---

### Visitor Pattern with Path Context

**Implementation**:
```rust
pub trait NodeVisitor {
    fn visit_node(&mut self, node: &Node, depth: usize, path: &[String]);
}

impl NodeVisitor for TextExtractor {
    fn visit_node(&mut self, node: &Node, _depth: usize, path: &[String]) {
        if let Node::Text { id, characters, .. } = node {
            let hierarchy = build_hierarchy_path(path);
            self.texts.push(ExtractedText {
                node_id: id.clone(),
                text: characters.clone(),
                path: hierarchy,
                sequence_number: self.sequence_number,
            });
            self.sequence_number += 1;
        }
    }
}
```

**Why**: Decouples traversal from extraction logic. Adding new extractors (images, components, styles) requires only implementing `NodeVisitor` trait, no changes to traversal code.

**Location**: `src/service/traversal.rs` lines 6-8, `src/extractor/text.rs` lines 36-76

---

## Development Tasks

### Add New API Endpoint

1. Define types in `src/models/document.rs`
2. Add method to `src/client/figma.rs`
3. Add command handler in `src/cli/commands.rs`
4. Update CLI args in `src/cli/args.rs`
5. Add test in `tests/integration_tests.rs`

### Add New Output Format

1. Add variant to `OutputFormat` in `src/cli/args.rs`
2. Implement formatter in `src/cli/output.rs`
3. Update `format_output` match statement
4. Add test case with sample data

### Add Configuration Field

1. Add field to struct in `src/core/config.rs`
2. Update `Default` implementation
3. Add validation in `validate()` method
4. Update merge logic if needed
5. Document in README.md config section

---

## Common Issues

### Issue: Token Not Found

**Symptom**: "No authentication token found" error

**Check**:
```bash
figma auth test
cat ~/.config/figma-cli/config.json
echo $FIGMA_TOKEN
```

**Fix**:
```bash
figma auth login
# or
export FIGMA_TOKEN="figd_..."
```

---

### Issue: Rate Limit Exceeded

**Symptom**: 429 errors, "Rate limit exceeded" messages

**Check**: Look for retry_after header in error response

**Fix**: Automatic retry with backoff is implemented. For persistent issues:
- Reduce `max_concurrent` in extract command
- Increase retry delays in `src/client/retry.rs`

---

### Issue: Large File Memory Usage

**Symptom**: High memory usage, potential OOM

**Check**:
```bash
figma extract FILE_KEY --depth 1  # Limit depth first
```

**Fix**:
- Use depth limiting (`--depth`)
- Filter by specific pages (`--pages`)
- Enable streaming mode (future feature)

---

## Key Constants

**Locations**:
- `src/core/config.rs`: default configurations
- `src/client/retry.rs`: retry behavior (max_retries, backoff)
- `src/client/figma.rs`: API constants (TIMEOUT, API_BASE)
- `src/core/performance.rs`: concurrency and caching settings

**To modify**: Edit constant or add to `Config` struct for runtime configuration

---

## Testing

### Run All Tests
```bash
cargo test
```

### Run Specific Test Module
```bash
cargo test client::
cargo test -- --nocapture  # See println! output
```

### Test Coverage
```bash
cargo llvm-cov test
```

### Integration Test with Real File
```bash
FIGMA_TOKEN=figd_xxx cargo run -- extract FILE_KEY --output test.json
```

---

This guide contains only implementation-critical knowledge. For user docs, see [README.md](README.md).