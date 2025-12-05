# Figma CLI

[![CI](https://github.com/junyeong-ai/figma-cli/workflows/CI/badge.svg)](https://github.com/junyeong-ai/figma-cli/actions)
[![Rust](https://img.shields.io/badge/rust-1.91.1%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-blue?style=flat-square)](https://github.com/junyeong-ai/figma-cli/releases)

> **🎨 Figma Design Extraction & Query CLI**

**🌐 English | [한국어](README.md)**

---

## ⚡ Key Features

- 🚀 **Automatic Caching** - Instant repeated tasks (0ms after first run)
- 🔍 **JMESPath Queries** - Complex data exploration
- 🖼️ **Image Extraction** - Base64 encoding support
- 📦 **Multiple Formats** - JSON, Markdown, Text output
- ⚙️ **Flexible Filtering** - Page and frame pattern matching

---

## 🚀 Quick Start

### 1. Installation

```bash
git clone https://github.com/junyeong-ai/figma-cli
cd figma-cli
./scripts/install.sh
```

### 2. Authentication

```bash
figma-cli auth login
```

**Get Token**: [Figma Settings](https://www.figma.com/settings) → Personal Access Tokens

### 3. Usage

```bash
figma-cli extract <FILE_KEY>
figma-cli query <FILE_KEY> "name"
figma-cli images <FILE_KEY> --frames "123:456"
```

---

## 📖 Commands

### `extract` - Design Extraction

```bash
# Basic extraction
figma-cli extract <FILE_KEY>

# URL support
figma-cli extract "https://figma.com/file/<FILE_KEY>/Design"

# Page filtering
figma-cli extract <FILE_KEY> --pages "Page 1,Page 2"
figma-cli extract <FILE_KEY> --page-pattern ".*Mobile.*"

# Frame filtering
figma-cli extract <FILE_KEY> --frame-pattern "^Component/.*"

# Output formats
figma-cli extract <FILE_KEY> --format json --output design.json
figma-cli extract <FILE_KEY> --format markdown --output design.md
figma-cli extract <FILE_KEY> --format text
figma-cli extract <FILE_KEY> --format summary    # AI-optimized markdown

# Pretty JSON
figma-cli extract <FILE_KEY> --pretty

# Include images
figma-cli extract <FILE_KEY> --with-images --image-dir ./images

# Include hidden nodes
figma-cli extract <FILE_KEY> --include-hidden
```

### `query` - JMESPath Queries

```bash
# Simple field
figma-cli query <FILE_KEY> "name"

# Array projection
figma-cli query <FILE_KEY> "document.children[*].name"

# Filtering
figma-cli query <FILE_KEY> "document.children[?name=='Cover']"

# Complex query
figma-cli query <FILE_KEY> "{fileName: name, version: version}" --pretty

# Specific node query
figma-cli query <FILE_KEY> --nodes "30:71,0:1" "nodes"

# Depth limiting
figma-cli query <FILE_KEY> "name" --depth 3
```

### `images` - Image Extraction

```bash
# Extract frames
figma-cli images <FILE_KEY> --frames "123:456,789:012"

# Specify format
figma-cli images <FILE_KEY> --frames "123:456" --format png
figma-cli images <FILE_KEY> --frames "123:456" --format svg
figma-cli images <FILE_KEY> --frames "123:456" --format pdf

# Scale adjustment
figma-cli images <FILE_KEY> --frames "123:456" --scale 2.0
figma-cli images <FILE_KEY> --frames "123:456" --scale 3.0

# Base64 encoding
figma-cli images <FILE_KEY> --frames "123:456" --base64

# Pretty JSON output
figma-cli images <FILE_KEY> --frames "123:456" --pretty
```

### `cache` - Cache Management

```bash
# Statistics
figma-cli cache stats

# List entries
figma-cli cache list
figma-cli cache list --json

# Clear cache
figma-cli cache clear --yes
```

### `inspect` - File Inspection

```bash
# Basic inspection
figma-cli inspect <FILE_KEY>

# Depth limiting
figma-cli inspect <FILE_KEY> --depth 2
```

### `auth` - Authentication

```bash
figma-cli auth login   # Save token
figma-cli auth test    # Test token
figma-cli auth logout  # Remove token
```

### `config` - Configuration

```bash
figma-cli config init  # Initialize config
figma-cli config show  # Show config
figma-cli config edit  # Edit config
```

---

## 💡 Use Cases

### AI Agents

```bash
# Extract design data
figma-cli extract <FILE_KEY> --output design.json

# Extract images with Base64
figma-cli images <FILE_KEY> --frames "123:456" --base64 --output images.json

# Extract only needed data with queries
figma-cli query <FILE_KEY> "{pages: document.children[*].name, meta: {name, version}}"
```

### Design Analysis

```bash
# Get all page names
figma-cli query <FILE_KEY> "document.children[*].name"

# Find pages matching pattern
figma-cli query <FILE_KEY> "document.children[?contains(name, 'Mobile')]"

# Statistics
figma-cli query <FILE_KEY> "length(document.children)"
```

---

## ⚙️ Configuration

### Priority Order

1. CLI arguments (`--format json`)
2. Environment variables (`FIGMA_TOKEN`)
3. Project config (`./figma-cli.toml`)
4. Global config (`~/.config/figma-cli/config.toml`)

### Config File

**Location**: `~/.config/figma-cli/config.toml`

```toml
token = "figd_..."

[extraction]
depth = 5
styles = true
components = true

[images]
scale = 2.0
format = "png"

[cache]
ttl = 24

[http]
timeout = 30
retries = 3
```

---

## 🎯 Performance

Actual test results (File key: kAP6ItdoLNNJ7HLOWMnCUf, depth=2):

| Operation | First Run | Cached |
|-----------|-----------|--------|
| Extract | 8754ms | 0ms |
| Query | ~3000ms | 22ms |

**Cache Location**: `~/Library/Caches/figma-cli` (macOS)

---

## 🛠️ Development

### Requirements

- Rust 1.91.1+ (2024 edition)

### Build & Test

```bash
cargo build --release
cargo test
cargo fmt
cargo clippy
```

---

## 📄 License

MIT OR Apache-2.0

---

## 📚 Documentation

- [CLAUDE.md](CLAUDE.md) - AI Agent Developer Guide
- [Figma API](https://www.figma.com/developers/api)

---

**Made with ❤️ and Rust**
