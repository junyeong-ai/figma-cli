#!/usr/bin/env bash
set -e

BINARY_NAME="figma"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
USER_SKILL_DIR="$HOME/.claude/skills/figma-design"

echo "🗑️  Uninstalling Figma CLI..."
echo

# ============================================================================
# Part 1: Remove Binary
# ============================================================================

if [ -f "$INSTALL_DIR/$BINARY_NAME" ]; then
    rm "$INSTALL_DIR/$BINARY_NAME"
    echo "✅ Removed $INSTALL_DIR/$BINARY_NAME"
else
    echo "⚠️  Binary not found at $INSTALL_DIR/$BINARY_NAME"
fi

# ============================================================================
# Part 2: Remove Config (Optional)
# ============================================================================

echo
read -p "Remove global configuration? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [ -d "$HOME/.config/figma-cli" ]; then
        rm -rf "$HOME/.config/figma-cli"
        echo "✅ Removed ~/.config/figma-cli"
    else
        echo "⚠️  Global config not found"
    fi
fi

# ============================================================================
# Part 3: Remove Skill (Optional, with Backup)
# ============================================================================

echo
if [ -d "$USER_SKILL_DIR" ]; then
    echo "📦 Claude Code skill detected at:"
    echo "   $USER_SKILL_DIR"
    echo
    read -p "Remove Claude Code skill? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Create backup before removing
        timestamp=$(date +%Y%m%d_%H%M%S)
        backup_dir="$USER_SKILL_DIR.backup_$timestamp"

        echo "📦 Creating backup: $backup_dir"
        cp -r "$USER_SKILL_DIR" "$backup_dir"

        rm -rf "$USER_SKILL_DIR"
        echo "✅ Removed $USER_SKILL_DIR"
        echo "   Backup saved at: $backup_dir"
    else
        echo "⏭️  Keeping Claude Code skill"
    fi
else
    echo "ℹ️  Claude Code skill not found (user-level)"
fi

# ============================================================================
# Part 4: Final Message
# ============================================================================

echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Uninstallation complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo
echo "Notes:"
echo "  • Project-level skill (.claude/skills/) is NOT removed"
echo "  • Local figma-cli.toml is NOT removed"
echo "  • Figma tokens in system keychain are NOT removed"
echo ""
echo "To completely remove all traces:"
echo "  1. Delete project directory"
echo "  2. Remove tokens from system keychain manually"
echo ""
