#!/bin/bash
# sync-crew.sh - Syncs agents from the My-Brain-Is-Full-Crew submodule to local providers

set -e

echo "=> Fetching latest crew submodule..."
git submodule update --remote --merge My-Brain-Is-Full-Crew

# Target directory: Prefer the first argument, otherwise attempt to find the vault path.
# Usage: ./sync-crew.sh [/path/to/vault]
VAULT_DEST="${1:-}"

if [ -z "$VAULT_DEST" ]; then
    # Detect OS to determine default Tauri app data location
    OS="$(uname)"
    case "$OS" in
        Darwin*)
            CONFIG_FILE="$HOME/Library/Application Support/com.ray2509.dis/vault_config.json"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            # Windows via Git Bash, MSYS2, or Cygwin
            if [ -n "$APPDATA" ]; then
                # Convert Windows path to Unix style for bash
                CONFIG_FILE="$(echo "$APPDATA" | sed 's/\\/\//g' | sed 's/\([A-Z]\):/\/\L\1/')/com.ray2509.dis/vault_config.json"
            else
                CONFIG_FILE="$HOME/AppData/Roaming/com.ray2509.dis/vault_config.json"
            fi
            ;;
        *)
            # Fallback for Linux or other Unix-like environments
            CONFIG_FILE="$HOME/.local/share/com.ray2509.dis/vault_config.json"
            ;;
    esac

    if [ -f "$CONFIG_FILE" ]; then
        VAULT_DEST=$(grep -o '"vault_path": "[^"]*' "$CONFIG_FILE" | cut -d'"' -f4)
    fi
fi

if [ -z "$VAULT_DEST" ] || [ ! -d "$VAULT_DEST" ]; then
    echo "!! Error: Could not determine valid Vault path. Please provide it as an argument."
    echo "Usage: ./sync-crew.sh /path/to/your/obsidian/vault"
    exit 1
fi

echo "=> Target Vault found: $VAULT_DEST"
echo "=> Creating vault agent directories..."
mkdir -p "$VAULT_DEST/.claude" "$VAULT_DEST/.gemini" "$VAULT_DEST/.codex"

echo "=> Syncing Claude agent directories..."
rsync -a --delete My-Brain-Is-Full-Crew/agents/ "$VAULT_DEST/.claude/agents/"
rsync -a --delete My-Brain-Is-Full-Crew/skills/ "$VAULT_DEST/.claude/skills/"
rsync -a --delete My-Brain-Is-Full-Crew/references/ "$VAULT_DEST/.claude/references/"
cp My-Brain-Is-Full-Crew/DISPATCHER.md "$VAULT_DEST/CLAUDE.md"

echo "=> Syncing Gemini agent directories..."
rsync -a --delete My-Brain-Is-Full-Crew/agents/ "$VAULT_DEST/.gemini/agents/"
rsync -a --delete My-Brain-Is-Full-Crew/skills/ "$VAULT_DEST/.gemini/skills/"
rsync -a --delete My-Brain-Is-Full-Crew/references/ "$VAULT_DEST/.gemini/references/"
cp My-Brain-Is-Full-Crew/DISPATCHER.md "$VAULT_DEST/GEMINI.md"

echo "=> Syncing Codex agent directories..."
rsync -a --delete My-Brain-Is-Full-Crew/agents/ "$VAULT_DEST/.codex/agents/"
rsync -a --delete My-Brain-Is-Full-Crew/skills/ "$VAULT_DEST/.codex/skills/"
rsync -a --delete My-Brain-Is-Full-Crew/references/ "$VAULT_DEST/.codex/references/"
cp My-Brain-Is-Full-Crew/DISPATCHER.md "$VAULT_DEST/CODEX.md"

echo "=> Sync complete! Vault-centric .claude, .gemini and .codex folders updated."
