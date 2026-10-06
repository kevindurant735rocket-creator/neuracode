#!/bin/bash

set -e

# NeuraCode Installation Script

echo "🧠 NeuraCode Installer"
echo "======================"
echo ""

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

case $OS in
    linux)
        TARGET="${ARCH}-unknown-linux-gnu"
        ;;
    darwin)
        TARGET="${ARCH}-apple-darwin"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please install from source: cargo install neuracode-cli"
        exit 1
        ;;
esac

echo "Detected: $OS ($ARCH)"
echo "Target: $TARGET"
echo ""

# Check if cargo is installed
if command -v cargo &> /dev/null; then
    echo "📦 Installing via cargo..."
    cargo install neuracode-cli
    echo ""
    echo "✅ NeuraCode installed successfully!"
    exit 0
fi

# Download pre-built binary
VERSION="0.1.0"
URL="https://github.com/neuracode/neuracode/releases/download/v${VERSION}/neuracode-${TARGET}.tar.gz"

echo "📥 Downloading NeuraCode v${VERSION}..."
echo "URL: $URL"
echo ""

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download
if command -v curl &> /dev/null; then
    curl -L "$URL" -o "$TMP_DIR/neuracode.tar.gz"
elif command -v wget &> /dev/null; then
    wget "$URL" -O "$TMP_DIR/neuracode.tar.gz"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Extract
echo "📦 Extracting..."
tar -xzf "$TMP_DIR/neuracode.tar.gz" -C "$TMP_DIR"

# Install
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "📁 Installing to $INSTALL_DIR..."
cp "$TMP_DIR/neuracode" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/neuracode"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  $INSTALL_DIR is not in your PATH"
    echo "Add the following to your shell profile:"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "✅ NeuraCode installed successfully!"
echo ""
echo "Quick start:"
echo "  neuracode init      # Initialize your project"
echo "  neuracode index     # Index your codebase"
echo "  neuracode search    # Search your code"
echo ""
