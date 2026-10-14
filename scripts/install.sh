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
        EXT="tar.gz"
        ;;
    darwin)
        TARGET="${ARCH}-apple-darwin"
        EXT="tar.gz"
        ;;
    mingw*|msys*|cygwin*)
        OS="windows"
        TARGET="x86_64-pc-windows-msvc"
        EXT="zip"
        ;;
    *)
        echo "Unsupported OS: $OS"
        echo "Please install from source: cargo install --path crates/neuracode-cli"
        exit 1
        ;;
esac

echo "Detected: $OS ($ARCH)"
echo "Target: $TARGET"
echo ""

# Check if cargo is installed
if command -v cargo &> /dev/null; then
    echo "📦 Installing via cargo..."
    if [ -d "crates/neuracode-cli" ]; then
        # Running from a source checkout: install local code (works offline)
        cargo install --path crates/neuracode-cli
    elif cargo install neuracode-cli; then
        # Published crate installed
        :
    else
        echo "crates.io install failed; falling back to pre-built binary..."
        # fall through to binary download below (do not exit)
        SKIP_CARGO_EXIT=1
    fi
    if [ -z "$SKIP_CARGO_EXIT" ]; then
        echo ""
        echo "✅ NeuraCode installed successfully!"
        exit 0
    fi
fi

# Download pre-built binary (matches .github/workflows/release.yml matrix)
ORG="kevindurant735rocket-creator"
VERSION="0.1.0"
URL="https://github.com/${ORG}/neuracode/releases/download/v${VERSION}/neuracode-${TARGET}.${EXT}"

echo "📥 Downloading NeuraCode v${VERSION}..."
echo "URL: $URL"
echo ""

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download
if command -v curl &> /dev/null; then
    curl -L "$URL" -o "$TMP_DIR/neuracode.$EXT"
elif command -v wget &> /dev/null; then
    wget "$URL" -O "$TMP_DIR/neuracode.$EXT"
else
    echo "Error: curl or wget is required"
    exit 1
fi

# Extract
echo "📦 Extracting..."
if [ "$EXT" = "zip" ]; then
    unzip -o "$TMP_DIR/neuracode.$EXT" -d "$TMP_DIR"
    BIN_NAME="neuracode.exe"
else
    tar -xzf "$TMP_DIR/neuracode.$EXT" -C "$TMP_DIR"
    BIN_NAME="neuracode"
fi

# Install
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "📁 Installing to $INSTALL_DIR..."
cp "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BIN_NAME"

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
