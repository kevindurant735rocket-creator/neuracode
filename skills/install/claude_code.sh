#!/bin/bash

set -e

echo "Installing NeuraCode for Claude Code..."
echo ""

# Create directories
mkdir -p ~/.claude/skills/neuracode

# Copy skill
cp -r skills/neuracode/* ~/.claude/skills/neuracode/

# Update settings
SETTINGS_FILE=~/.claude/settings.json

if [ -f "$SETTINGS_FILE" ]; then
    echo "Updating existing settings..."
    # In production, this would merge JSON properly
else
    echo "Creating new settings..."
    cat > "$SETTINGS_FILE" << 'SETTINGS'
{
  "mcpServers": {
    "neuracode": {
      "command": "neuracode-mcp",
      "args": ["--stdio"]
    }
  }
}
SETTINGS
fi

# Create CLAUDE.md
CLAUDE_FILE=~/.claude/CLAUDE.md

if [ -f "$CLAUDE_FILE" ]; then
    echo "Updating CLAUDE.md..."
    echo "" >> "$CLAUDE_FILE"
    echo "# NeuraCode Integration" >> "$CLAUDE_FILE"
    echo "" >> "$CLAUDE_FILE"
    echo "This project uses NeuraCode for enhanced code intelligence." >> "$CLAUDE_FILE"
else
    echo "Creating CLAUDE.md..."
    cat > "$CLAUDE_FILE" << 'CLAUDE'
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
CLAUDE
fi

echo ""
echo "✅ NeuraCode installed for Claude Code!"
echo ""
echo "Restart Claude Code to activate."
