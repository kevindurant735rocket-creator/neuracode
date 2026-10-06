#!/bin/bash

set -e

echo "Installing NeuraCode for Cursor..."
echo ""

# Create directories
mkdir -p ~/.cursor/rules

# Update MCP config
MCP_FILE=~/.cursor/mcp.json

if [ -f "$MCP_FILE" ]; then
    echo "Updating existing MCP config..."
else
    echo "Creating new MCP config..."
    cat > "$MCP_FILE" << 'MCP'
{
  "mcpServers": {
    "neuracode": {
      "command": "neuracode-mcp",
      "args": ["--stdio"]
    }
  }
}
MCP
fi

# Create rules
RULES_FILE=~/.cursor/rules/neuracode.md

echo "Creating rules..."
cat > "$RULES_FILE" << 'RULES'
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
RULES

echo ""
echo "✅ NeuraCode installed for Cursor!"
echo ""
echo "Restart Cursor to activate."
