#!/bin/bash
# ◈ Ascii-Vault Automated Installer

REPO="lorediggia/ascii-vault"
BIN_NAME="ascii-vault"
BIN_DIR="$HOME/.local/bin"

mkdir -p "$BIN_DIR"

echo "◈ Fetching latest release from GitHub..."

URL=$(curl -s https://api.github.com/repos/$REPO/releases/latest | \
      grep "browser_download_url" | \
      cut -d '"' -f 4)

if [ -z "$URL" ]; then
    echo "✗ Error: No binary found in releases. Please check the repo URL or compile with cargo."
    exit 1
fi

echo "◈ Downloading $BIN_NAME..."
curl -L "$URL" -o "$BIN_DIR/$BIN_NAME"

chmod +x "$BIN_DIR/$BIN_NAME"

echo "---"
echo "✓ Success: $BIN_NAME installed to $BIN_DIR"

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo "◈ Note: $BIN_DIR is not in your PATH."
    echo "  Add this line to your .bashrc or .zshrc:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "◈ Launch with: $BIN_NAME"
