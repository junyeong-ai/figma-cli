---
name: figma-design
version: 0.1.0
description: Figma CLI for extracting designs, querying data with JMESPath, and generating images. Automatic caching for instant repeated tasks.
allowed-tools: [Bash, Read, Grep, Glob]
---

# Figma CLI Expert

Extract designs, query with JMESPath, manage cache, generate images.

## Quick Start

```bash
figma-cli auth login        # Get token from https://www.figma.com/settings
figma-cli config show       # Verify config
```

## Commands

### Extract

```bash
# Basic (accepts URL or file key)
figma-cli extract FILE_KEY --depth 3 --output design.json --pretty

# Filtering
figma-cli extract FILE_KEY --pages "Page 1,Page 2"
figma-cli extract FILE_KEY --page-pattern ".*Mobile.*"
figma-cli extract FILE_KEY --frame-pattern "^Component/.*"

# Formats
figma-cli extract FILE_KEY --format json --output design.json
figma-cli extract FILE_KEY --format markdown --output design.md
figma-cli extract FILE_KEY --format text

# Options
figma-cli extract FILE_KEY --with-images --image-dir ./images
figma-cli extract FILE_KEY --include-hidden
```

### Query

```bash
# JMESPath queries (automatic caching)
figma-cli query FILE_KEY "name"
figma-cli query FILE_KEY "document.children[*].name"
figma-cli query FILE_KEY "document.children[?name=='Cover']"
figma-cli query FILE_KEY "{fileName: name, version: version}" --pretty

# Specific nodes
figma-cli query FILE_KEY --nodes "30:71,0:1" "nodes"

# Control depth
figma-cli query FILE_KEY "name" --depth 3
```

### Inspect

```bash
# URL or node IDs
figma-cli inspect "https://www.figma.com/design/FILE_KEY/?node-id=9845-142737"
figma-cli inspect FILE_KEY --nodes "123:456,789:012" --depth 2
```

### Images

```bash
# URL mode (fast, S3 URLs)
figma-cli images FILE_KEY --frames "9845:142737"

# Base64 mode (for AI)
figma-cli images FILE_KEY --frames "9845:142737" --base64

# Options
figma-cli images FILE_KEY --frames "ID" --format svg --scale 3.0
figma-cli images FILE_KEY --frames "ID1,ID2,ID3" --base64
```

**Formats**: png, jpg, svg, pdf
**Scale**: 0.01 to 4.0 (default: 2.0)

### Cache

```bash
figma-cli cache stats                # Show statistics
figma-cli cache list                 # List entries
figma-cli cache list --json          # JSON format
figma-cli cache clear --yes          # Clear all
```

**Automatic caching** - all commands use cache transparently (no flags needed).

### Auth & Config

```bash
# Auth
figma-cli auth login
figma-cli auth test
figma-cli auth logout

# Config
figma-cli config init
figma-cli config show
figma-cli config edit
figma-cli config get token
figma-cli config set token "figd_..."
```

## Configuration

Priority: CLI args > ENV (`FIGMA_TOKEN`) > Project config (`./figma-cli.toml`) > Global config (`~/.config/figma-cli/config.toml`)

```toml
[auth]
token = "figd_..."

[extraction]
depth = 5

[images]
scale = 2.0
format = "png"

[cache]
ttl = 24
```

## JMESPath Examples

```bash
# Metadata
figma-cli query FILE_KEY "{fileName: name, version: version, pages: length(document.children)}"

# All page names
figma-cli query FILE_KEY "document.children[*].name"

# Find by name
figma-cli query FILE_KEY "document.children[?name=='MyPage']"

# Slice and project
figma-cli query FILE_KEY "document.children[0:3].{page: name, id: id}"

# Count
figma-cli query FILE_KEY "length(document.children)"
```

## URL Parsing

Supports Figma URLs:
- Extracts file keys automatically
- Converts node-id format (9845-142737 → 9845:142737)
- Works with `/file/` and `/design/` formats

## Common Tasks

```bash
# Extract with auto-cache
figma-cli extract FILE_KEY --depth 5 -o design.json --pretty

# Query metadata
figma-cli query FILE_KEY "{name: name, version: version}" --depth 2

# Get frame as base64 for AI
figma-cli images FILE_KEY --frames "9845:142737" --base64 -o frame.json

# Inspect component
figma-cli inspect FILE_KEY --nodes "COMPONENT_ID" --depth 2 --pretty
```
