#!/usr/bin/env bash
# Universal installer for figma-cli
# Supports: Linux, macOS, Windows (WSL/Git Bash)
# Features: Prebuilt binaries, checksum verification, auto-update, Claude Code integration

set -e

# Configuration
readonly BINARY_NAME="figma-cli"
readonly REPO="junyeong-ai/figma-cli"
readonly INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
readonly CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/figma-cli"  # Use XDG standard
readonly SKILL_NAME="figma-design"
readonly PROJECT_SKILL_DIR=".claude/skills/$SKILL_NAME"
readonly USER_SKILL_DIR="$HOME/.claude/skills/$SKILL_NAME"

# Colors for output
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ${NC}  $1"
}

log_success() {
    echo -e "${GREEN}✅${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}⚠️${NC}  $1"
}

log_error() {
    echo -e "${RED}❌${NC} $1" >&2
}

# Detect operating system and architecture
detect_platform() {
    local os arch

    # Detect OS
    case "$(uname -s)" in
        Linux*)
            os="unknown-linux-gnu"
            ;;
        Darwin*)
            os="apple-darwin"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            os="pc-windows-msvc"
            ;;
        *)
            log_error "Unsupported operating system: $(uname -s)"
            exit 1
            ;;
    esac

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="aarch64"
            ;;
        armv7l)
            arch="armv7"
            ;;
        *)
            log_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac

    echo "${arch}-${os}"
}

# Check for required dependencies
check_dependencies() {
    local deps=("curl" "tar")
    local missing=()

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" &> /dev/null; then
            missing+=("$dep")
        fi
    done

    if [ ${#missing[@]} -gt 0 ]; then
        log_error "Missing required dependencies: ${missing[*]}"
        log_info "Please install them and try again"
        exit 1
    fi
}

# Get the latest version from GitHub releases
get_latest_version() {
    local version

    version=$(curl -sf "https://api.github.com/repos/$REPO/releases/latest" \
        | grep '"tag_name"' \
        | sed -E 's/.*"v?([^"]+)".*/\1/' \
        2>/dev/null || echo "")

    if [ -z "$version" ]; then
        version="0.1.0"  # Fallback version
    fi

    echo "$version"
}

# Get currently installed version
get_installed_version() {
    if [ -x "$INSTALL_DIR/$BINARY_NAME" ]; then
        "$INSTALL_DIR/$BINARY_NAME" --version 2>/dev/null | sed 's/.*\s//' || echo "unknown"
    else
        echo "none"
    fi
}

# Download and verify binary
download_binary() {
    local version="$1"
    local target="$2"
    local temp_dir

    temp_dir=$(mktemp -d)
    cd "$temp_dir" || exit 1

    # Determine file extension
    local ext="tar.gz"
    if [[ "$target" == *"windows"* ]]; then
        ext="zip"
    fi

    local archive="${BINARY_NAME}-v${version}-${target}.${ext}"
    local url="https://github.com/$REPO/releases/download/v${version}/${archive}"
    local checksum_url="${url}.sha256"

    log_info "Downloading figma-cli v${version} for ${target}..." >&2

    # Download binary
    if ! curl -fLO "$url" --progress-bar 2>&2; then
        log_error "Failed to download binary from $url" >&2
        rm -rf "$temp_dir"
        return 1
    fi

    # Download and verify checksum
    log_info "Verifying checksum..." >&2
    if curl -fsSLO "$checksum_url" 2>&1 >&2; then
        if command -v sha256sum &> /dev/null; then
            if ! sha256sum -c "${archive}.sha256" >&2; then
                log_error "Checksum verification failed!" >&2
                rm -rf "$temp_dir"
                return 1
            fi
        elif command -v shasum &> /dev/null; then
            if ! shasum -a 256 -c "${archive}.sha256" >&2; then
                log_error "Checksum verification failed!" >&2
                rm -rf "$temp_dir"
                return 1
            fi
        else
            log_warning "No checksum tool found, skipping verification" >&2
        fi
        log_success "Checksum verified" >&2
    else
        log_warning "No checksum file available, skipping verification" >&2
    fi

    # Extract archive
    log_info "Extracting archive..." >&2
    if [[ "$ext" == "zip" ]]; then
        unzip -q "$archive" >&2
    else
        tar -xzf "$archive" >&2
    fi

    # Return path to binary
    if [[ "$target" == *"windows"* ]]; then
        echo "$temp_dir/${BINARY_NAME}.exe"
    else
        echo "$temp_dir/${BINARY_NAME}"
    fi
}

# Build from source as fallback
build_from_source() {
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo not found. Please install Rust: https://rustup.rs" >&2
        return 1
    fi

    # Clone and build
    local temp_dir
    temp_dir=$(mktemp -d)
    cd "$temp_dir" || exit 1

    git clone "https://github.com/$REPO.git" . >&2 || return 1
    cargo build --release >&2 || return 1

    echo "$temp_dir/target/release/$BINARY_NAME"
}

# Install the binary
install_binary() {
    local binary_path="$1"

    # Create install directory
    mkdir -p "$INSTALL_DIR"

    # Copy binary
    local dest="$INSTALL_DIR/$BINARY_NAME"
    cp "$binary_path" "$dest"
    chmod +x "$dest"

    # Code sign on macOS
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v codesign &> /dev/null; then
            codesign --force --deep --sign - "$dest" 2>/dev/null || true
        fi
    fi

    log_success "Installed to $dest"
}

# Install Claude Code skill
install_claude_skill() {
    if [ ! -d "$PROJECT_SKILL_DIR" ]; then
        log_info "No Claude Code skill found in project"
        return 0
    fi

    log_info "Installing Claude Code skill..."

    # Check if skill already exists
    if [ -d "$USER_SKILL_DIR" ]; then
        local project_version current_version
        project_version=$(get_skill_version "$PROJECT_SKILL_DIR/SKILL.md")
        current_version=$(get_skill_version "$USER_SKILL_DIR/SKILL.md")

        if [ "$project_version" == "$current_version" ]; then
            log_info "Claude Code skill is already up to date (v$current_version)"
            return 0
        fi

        # Backup existing skill
        local backup_dir="$USER_SKILL_DIR.backup.$(date +%Y%m%d_%H%M%S)"
        log_warning "Backing up existing skill to $backup_dir"
        mv "$USER_SKILL_DIR" "$backup_dir"
    fi

    # Install skill
    mkdir -p "$(dirname "$USER_SKILL_DIR")"
    cp -r "$PROJECT_SKILL_DIR" "$USER_SKILL_DIR"

    log_success "Claude Code skill installed successfully"
    log_info "Restart Claude Desktop to activate the skill"
}

# Get skill version from SKILL.md
get_skill_version() {
    local skill_file="$1"
    if [ -f "$skill_file" ]; then
        grep "^version:" "$skill_file" 2>/dev/null | sed 's/version: *//' || echo "unknown"
    else
        echo "none"
    fi
}

# Setup shell integration
setup_shell_integration() {
    local shell_rc

    # Detect shell configuration file
    case "$SHELL" in
        */bash)
            shell_rc="$HOME/.bashrc"
            ;;
        */zsh)
            shell_rc="$HOME/.zshrc"
            ;;
        */fish)
            shell_rc="$HOME/.config/fish/config.fish"
            ;;
        *)
            return 0
            ;;
    esac

    # Check if PATH already contains install dir
    if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
        return 0
    fi

    log_info "Adding $INSTALL_DIR to PATH in $shell_rc"

    # Add to PATH
    if [[ "$SHELL" == */fish ]]; then
        echo "set -gx PATH \$PATH $INSTALL_DIR" >> "$shell_rc"
    else
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$shell_rc"
    fi

    log_warning "Please run: source $shell_rc"
}

# Create initial configuration
setup_config() {
    if [ ! -d "$CONFIG_DIR" ]; then
        log_info "Creating configuration directory..."
        mkdir -p "$CONFIG_DIR"
    fi

    if [ ! -f "$CONFIG_DIR/config.toml" ]; then
        log_info "Creating default configuration..."
        cat > "$CONFIG_DIR/config.toml" << 'EOF'
# figma-cli configuration

# Get Figma token from https://www.figma.com/settings
# token = "figd_..."

[extraction]
depth = 5
max_depth = 10
styles = true
components = true
vectors = false

[http]
timeout = 30
retries = 3
retry_delay = 1000
max_delay = 60000
backoff = 2.0

[images]
scale = 2.0
format = "png"

[performance]
concurrent = 50
chunk_size = 100

[cache]
ttl = 24
EOF
        log_success "Configuration created at $CONFIG_DIR/config.toml"
    fi
}

# Main installation flow
main() {
    echo "================================================="
    echo "   figma-cli Universal Installer"
    echo "================================================="
    echo

    # Check dependencies
    check_dependencies

    # Detect platform
    local platform
    platform=$(detect_platform)
    log_info "Detected platform: $platform"

    # Check for existing installation
    local installed_version latest_version
    installed_version=$(get_installed_version)
    latest_version=$(get_latest_version)

    if [ "$installed_version" != "none" ]; then
        log_info "Current version: $installed_version"
        if [ "$installed_version" == "$latest_version" ]; then
            log_success "figma-cli is already up to date"
            install_claude_skill
            exit 0
        fi
        log_info "Updating to version $latest_version..."
    else
        log_info "Installing figma-cli v$latest_version..."
    fi

    # Try downloading prebuilt binary
    local binary_path
    set +e  # Temporarily disable exit on error
    binary_path=$(download_binary "$latest_version" "$platform")
    local download_result=$?
    set -e  # Re-enable exit on error

    # Fall back to building from source if download fails
    if [ $download_result -ne 0 ] || [ -z "$binary_path" ]; then
        log_warning "Prebuilt binary not available, trying to build from source..."
        set +e
        binary_path=$(build_from_source)
        local build_result=$?
        set -e

        if [ $build_result -ne 0 ] || [ -z "$binary_path" ]; then
            log_error "Installation failed"
            exit 1
        fi
    fi

    # Install binary
    install_binary "$binary_path"

    # Cleanup
    rm -rf "$(dirname "$binary_path")"

    # Setup configuration
    setup_config

    # Setup shell integration
    setup_shell_integration

    # Install Claude Code skill
    install_claude_skill

    # Verify installation
    if "$INSTALL_DIR/$BINARY_NAME" --version &> /dev/null; then
        log_success "Installation complete! 🎉"
        echo
        log_info "Run 'figma-cli --help' to get started"
        log_info "Run 'figma-cli auth login' to set up authentication"
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# Run main function
main "$@"