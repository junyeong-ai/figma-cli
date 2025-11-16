---
name: figma-design
version: 0.2.0
description: Figma CLI for extracting designs, inspecting nodes, and generating images. Use for design documentation, UI analysis, component extraction, or AI-powered design review. Triggers - Figma files, design systems, UI screenshots, frame extraction.
allowed-tools: [Bash, Read, Grep, Glob]
---

# Figma CLI Expert

Extract designs, inspect nodes, and generate images from Figma files.

## Quick Start

```bash
# Check installation
figma-cli --version

# Authenticate (get token from https://www.figma.com/settings)
figma-cli auth login

# Check config
figma-cli config show
```

## Commands

### 1. Extract - Get all text and structure

```bash
# From URL (auto-parses file key)
figma-cli extract "https://www.figma.com/design/kAP6ItdoLNNJ7HLOWMnCUf/..."

# Filter by page
figma-cli extract FILE_KEY --pages "Page 1,Page 2"
figma-cli extract FILE_KEY --page-pattern ".*Mobile.*"

# Control depth (default: 5)
figma-cli extract FILE_KEY --depth 3

# Output
figma-cli extract FILE_KEY --output design.json --pretty
```

**Returns**: JSON with metadata, structure, texts, and styles

### 2. Inspect - Get specific node details

```bash
# From URL (auto-extracts node-id)
figma-cli inspect "https://www.figma.com/design/FILE_KEY/?node-id=9845-142737"

# Multiple nodes
figma-cli inspect FILE_KEY --nodes "123:456,789:012" --depth 2

# Output
figma-cli inspect FILE_KEY --nodes "123:456" --output node.json --pretty
```

**Depth**: 0=node only, 1=with children, 2=with grandchildren

### 3. Images - Export frames

```bash
# URL mode (fast, returns S3 URLs)
figma-cli images FILE_KEY --frames "9845:142737"

# Base64 mode (for AI agents)
figma-cli images FILE_KEY --frames "9845:142737" --base64

# Custom format and scale
figma-cli images FILE_KEY --frames "ID" --format svg --scale 3.0
```

**Formats**: png, jpg, svg, pdf
**Scale**: 0.01 to 4.0 (default: 2.0)

### 4. Auth - Manage authentication

```bash
figma-cli auth login    # Store token
figma-cli auth test     # Validate token
figma-cli auth logout   # Remove token
```

### 5. Config - Manage settings

```bash
figma-cli config init           # Initialize config
figma-cli config show           # Display current settings
figma-cli config edit           # Edit with $EDITOR
figma-cli config path           # Show config file paths
figma-cli config get token      # Get specific value
figma-cli config set token "figd_..."  # Set specific value
```

## Configuration

Priority order (highest to lowest):
1. CLI arguments (`--token`)
2. Environment (`FIGMA_TOKEN`)
3. Project config (`./figma-cli.toml`)
4. Global config (`~/.config/figma-cli/config.toml`)

Example config:
```toml
[auth]
token = "figd_..."

[extract]
default_depth = 5

[images]
default_format = "png"
default_scale = 2.0
base64_enabled = false
```

## URL Parsing

The CLI automatically handles Figma URLs:
- Extracts file keys from URLs
- Converts node-id format (9845-142737 → 9845:142737)
- Supports both `/file/` and `/design/` URL formats

## Common Patterns

### Extract entire design
```bash
figma-cli extract FILE_KEY --depth 5 --output design.json --pretty
```

### Get specific frame for AI analysis
```bash
figma-cli images FILE_KEY --frames "9845:142737" --base64 --output frame.json
```

### Inspect component structure
```bash
figma-cli inspect FILE_KEY --nodes "COMPONENT_ID" --depth 2 --pretty
```

### Batch process multiple frames
```bash
figma-cli images FILE_KEY --frames "ID1,ID2,ID3" --base64
```

## Error Handling

- **No token**: Run `figma-cli auth login`
- **Invalid file key**: Check URL/key format
- **Node not found**: Verify node ID exists
- **Network errors**: Auto-retries 3 times with backoff

## Notes

- Korean/Unicode text fully supported
- Images: URL mode is fast, Base64 mode is slow but AI-compatible
- Use `--pretty` for readable output during debugging
- Depth affects memory usage - start with 3, increase if needed
