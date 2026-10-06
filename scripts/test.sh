#!/bin/bash

set -e

echo "🧪 Testing NeuraCode"
echo "==================="
echo ""

# Run Rust tests
echo "📦 Running Rust tests..."
cargo test --verbose

# Run Python tests
echo "🐍 Running Python tests..."
cd python
pytest -v
cd ..

echo ""
echo "✅ All tests passed!"
