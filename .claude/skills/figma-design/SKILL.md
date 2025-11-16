---
name: figma-design
version: 0.1.0
description: Extract Figma designs, query with JMESPath, generate images with base64 encoding. Automatic caching for instant repeated operations. Use when working with Figma files, design data extraction, querying design metadata, or generating design assets for AI processing.
allowed-tools: [Bash, Read, Grep, Glob]
---

# Figma CLI Expert

Extract designs, query with JMESPath, manage cache, generate images.

## Quick Start

```bash
figma-cli auth login
figma-cli config show
```

## Commands

### Extract

```bash
# Basic
figma-cli extract FILE_KEY --depth 3 -o design.json --pretty

# Filtering
figma-cli extract FILE_KEY --pages "Page 1,Page 2"
figma-cli extract FILE_KEY --page-pattern ".*Mobile.*"
figma-cli extract FILE_KEY --frame-pattern "^Component/.*"

# Format options
figma-cli extract FILE_KEY --format markdown -o design.md

# With images
figma-cli extract FILE_KEY --with-images --image-dir ./images --include-hidden
```

### Query

```bash
# Basic queries (always specify --depth to avoid "Request too large" errors)
figma-cli query FILE_KEY "name" --depth 1
figma-cli query FILE_KEY "document.children[*].name" --depth 2
figma-cli query FILE_KEY "document.children[?name=='Cover']" --depth 2
figma-cli query FILE_KEY "{fileName: name, version: version}" --depth 2 --pretty

# Specific nodes
figma-cli query FILE_KEY --nodes "30:71,0:1" "nodes" --depth 1
```

### Inspect

```bash
figma-cli inspect FILE_KEY --nodes "NODE_ID" --depth 2
figma-cli inspect "https://www.figma.com/design/FILE_KEY/?node-id=0-1"
```

### Images

```bash
figma-cli images FILE_KEY --frames "FRAME_ID"
figma-cli images FILE_KEY --frames "FRAME_ID" --base64
figma-cli images FILE_KEY --frames "FRAME_ID" --format svg --scale 3.0
```

Formats: png, jpg, svg, pdf | Scale: 0.01-4.0 (default: 2.0)

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
figma-cli config get extraction.depth
figma-cli config set token "figd_..."
```

## Configuration

**Priority**: CLI args > `FIGMA_TOKEN` env > `./figma-cli.toml` > `~/.config/figma-cli/config.toml`

```toml
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
# Object construction with functions
figma-cli query FILE_KEY "{fileName: name, version: version, pages: length(document.children)}" --depth 2

# Slice and projection
figma-cli query FILE_KEY "document.children[0:3].{page: name, id: id}" --depth 2

# Count function
figma-cli query FILE_KEY "length(document.children)" --depth 2
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
