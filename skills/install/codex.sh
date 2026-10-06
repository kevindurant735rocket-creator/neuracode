#!/bin/bash

set -e

echo "Installing NeuraCode for Codex..."
echo ""

# Create directories
mkdir -p ~/.codex

# Update config
CONFIG_FILE=~/.codex/config.toml

if [ -f "$CONFIG_FILE" ]; then
    echo "Updating existing config..."
    echo "" >> "$CONFIG_FILE"
    echo "# NeuraCode MCP Server" >> "$CONFIG_FILE"
    echo "[mcp_servers.neuracode]" >> "$CONFIG_FILE"
    echo 'command = "neuracode-mcp"' >> "$CONFIG_FILE"
    echo 'args = ["--stdio"]' >> "$CONFIG_FILE"
else
    echo "Creating new config..."
    cat > "$CONFIG_FILE" << 'CONFIG'
# NeuraCode MCP Server
[mcp_servers.neuracode]
command = "neuracode-mcp"
args = ["--stdio"]
CONFIG
fi

# Create AGENTS.md
AGENTS_FILE=~/.codex/AGENTS.md

echo "Creating AGENTS.md..."
cat > "$AGENTS_FILE" << 'AGENTS'
# NeuraCode Integration

This project uses NeuraCode for enhanced code intelligence.

## Available Tools

- `neuracode_search` - Semantic search across the codebase
- `neuracode_predict` - Predict context for tasks
- `neuracode_impact` - Analyze impact of changes
- `neuracode_architecture` - Get architecture information

## Usage

NeuraCode is automatically activated when you:
- Search for code
- Make changes to code
- Ask questions about the codebase
AGENTS

echo ""
echo "✅ NeuraCode installed for Codex!"
echo ""
echo "Restart Codex to activate."
