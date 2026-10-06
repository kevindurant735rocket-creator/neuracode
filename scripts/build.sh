#!/bin/bash

set -e

echo "🔨 Building NeuraCode"
echo "===================="
echo ""

# Build Rust
echo "📦 Building Rust components..."
cargo build --release

# Build Python
echo "🐍 Building Python module..."
cd python
pip install -e .
cd ..

echo ""
echo "✅ Build complete!"
echo ""
echo "Binary location: target/release/neuracode"
