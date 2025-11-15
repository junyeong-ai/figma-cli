# figma-cli

Fast Figma CLI for design extraction and AI-powered analysis

## ✨ Key Features

- **High-Speed Extraction**: 8x faster JSON parsing with Rust, 90% memory reduction
- **AI-Friendly**: Perfect compatibility with AI agents via Base64 image encoding
- **Flexible Filtering**: Regex filtering by pages and frames
- **Multiple Outputs**: JSON, Text, Markdown format support
- **Hierarchical Config**: Project → Global → Environment → CLI priority

## 🚀 Quick Start

### Installation

```bash
# Cargo
cargo install figma-cli

# Build from source
git clone https://github.com/junyeong-ai/figma-cli
cd figma-cli
./scripts/install.sh
```

### Configuration

```bash
# Set up token
figma auth login
# or
export FIGMA_TOKEN="figd_your_token_here"

# Initialize config
figma config init
```

### Basic Usage

```bash
# Extract entire file
figma extract https://www.figma.com/design/FILE_KEY/

# Extract specific pages
figma extract FILE_KEY --pages "Page 1,Page 2"

# JSON output (pretty)
figma extract FILE_KEY --pretty --output design.json

# Extract images (Base64)
figma images FILE_KEY --frames "123:456,789:012" --base64
```

## 📖 Main Commands

| Command | Description | Example |
|---------|-------------|---------|
| `extract` | Extract design content | `figma extract FILE_KEY` |
| `inspect` | Inspect specific nodes | `figma inspect FILE_KEY --nodes "123:456"` |
| `images` | Extract images | `figma images FILE_KEY --base64` |
| `auth` | Manage authentication | `figma auth login` |
| `config` | Manage configuration | `figma config show` |

## ⚙️ Configuration

Configuration priority:
1. CLI arguments (`--token`)
2. Environment variables (`FIGMA_TOKEN`)
3. Project config (`./figma-cli.toml`)
4. Global config (`~/.config/figma-cli/config.toml`)

Example config file:

```toml
[auth]
token = "figd_your_token_here"

[extract]
default_depth = 5

[images]
default_format = "png"
default_scale = 2.0
base64_enabled = false
```

## 💡 Advanced Usage

### Regex Filtering

```bash
# Extract only mobile-related pages
figma extract FILE_KEY --page-pattern ".*Mobile.*"

# Extract only component frames
figma extract FILE_KEY --frame-pattern "^Component/.*"
```

### AI Agent Integration

```bash
# Extract with Base64 encoded images
figma extract FILE_KEY --with-images --output design.json
figma images FILE_KEY --base64 --output images.json

# Integrate with Claude or GPT
cat design.json | your-ai-tool process
```

### Batch Processing

```bash
# Process multiple files
for file_key in FILE1 FILE2 FILE3; do
  figma extract $file_key --output "${file_key}.json"
done
```

## 🤝 Contributing

To contribute to this project:

1. Fork & Clone
2. Create branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push (`git push origin feature/amazing-feature`)
5. Create Pull Request

## 📄 License

MIT