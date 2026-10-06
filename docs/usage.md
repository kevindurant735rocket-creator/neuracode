# NeuraCode Usage Guide

## Table of Contents

1. [Installation](#installation)
2. [Quick Start](#quick-start)
3. [Commands](#commands)
4. [Configuration](#configuration)
5. [AI Agent Integration](#ai-agent-integration)
6. [Advanced Usage](#advanced-usage)

## Installation

### From Source

```bash
git clone https://github.com/neuracode/neuracode.git
cd neuracode
cargo build --release
```

### From Package Manager

```bash
# macOS/Linux
curl -fsSL https://raw.githubusercontent.com/neuracode/neuracode/main/install.sh | bash

# Cargo
cargo install neuracode-cli
```

## Quick Start

### 1. Initialize Your Project

```bash
cd your-project
neuracode init
```

This creates a `.neuracode` directory with configuration.

### 2. Index Your Codebase

```bash
neuracode index
```

This analyzes your code and builds a knowledge graph.

### 3. Search Your Code

```bash
neuracode search "authentication"
```

### 4. Install for AI Agents

```bash
neuracode install --all
```

## Commands

### `neuracode init`

Initialize NeuraCode in the current project.

```bash
neuracode init [path]
```

**Options:**
- `path`: Project path (default: current directory)

### `neuracode index`

Index a codebase.

```bash
neuracode index [path] [--force]
```

**Options:**
- `path`: Project path (default: current directory)
- `--force`: Force re-indexing

### `neuracode search`

Search the codebase.

```bash
neuracode search <query> [--limit N] [--format text|json]
```

**Options:**
- `--limit`: Maximum number of results (default: 10)
- `--format`: Output format (default: text)

**Examples:**
```bash
# Search for authentication code
neuracode search "authentication"

# Get JSON output
neuracode search "user login" --format json

# Limit results
neuracode search "database" --limit 5
```

### `neuracode predict`

Predict context for a task.

```bash
neuracode predict <task> [--format text|json]
```

**Examples:**
```bash
# Predict context for bug fix
neuracode predict "fix login bug"

# Predict context for refactoring
neuracode predict "refactor user module"
```

### `neuracode impact`

Analyze impact of changes.

```bash
neuracode impact <target> [--format text|json]
```

**Examples:**
```bash
# Analyze impact of changing a file
neuracode impact src/auth.ts

# Analyze impact of changing a function
neuracode impact "loginUser"
```

### `neuracode architecture`

Show architecture information.

```bash
neuracode architecture [--format text|json]
```

### `neuracode hotspots`

Identify codebase hotspots.

```bash
neuracode hotspots [--limit N] [--format text|json]
```

### `neuracode understand`

Understand an image.

```bash
neuracode understand <path> [--format text|json]
```

**Examples:**
```bash
# Understand architecture diagram
neuracode understand diagrams/architecture.png

# Parse flowchart
neuracode understand diagrams/flowchart.png
```

### `neuracode install`

Install NeuraCode for AI agents.

```bash
neuracode install [--agent <name>] [--all]
```

**Options:**
- `--agent`: Install for specific agent
- `--all`: Install for all supported agents

**Examples:**
```bash
# Install for all agents
neuracode install --all

# Install for Claude Code
neuracode install --agent claude-code

# Install for Cursor
neuracode install --agent cursor
```

### `neuracode uninstall`

Uninstall NeuraCode from AI agents.

```bash
neuracode uninstall [--agent <name>] [--all]
```

### `neuracode stats`

Show statistics.

```bash
neuracode stats
```

### `neuracode config`

Show configuration.

```bash
neuracode config [--full]
```

## Configuration

NeuraCode uses a TOML configuration file at `.neuracode/config.toml`.

```toml
# Cache settings
cache_size_mb = 512

# Feature flags
enable_prediction = true
enable_learning = true
enable_multimodal = true

# Supported languages
languages = ["rust", "javascript", "typescript", "python", "go", "java", "c", "cpp"]

# Ignore patterns
ignore_patterns = [
    "node_modules",
    ".git",
    "target",
    "dist",
    "build",
    "__pycache__",
    ".venv",
]

# Maximum file size to index (in bytes)
max_file_size = 1048576
```

## AI Agent Integration

### Claude Code

```bash
neuracode install --agent claude-code
```

This creates:
- `~/.claude/settings.json` with MCP server config
- `~/.claude/CLAUDE.md` with NeuraCode instructions
- `~/.claude/skills/neuracode/` with skill files

### Cursor

```bash
neuracode install --agent cursor
```

This creates:
- `~/.cursor/mcp.json` with MCP server config
- `~/.cursor/rules` with NeuraCode instructions

### Codex

```bash
neuracode install --agent codex
```

This creates:
- `~/.codex/config.toml` with MCP server config
- `~/.codex/AGENTS.md` with NeuraCode instructions

## Advanced Usage

### Semantic Search

```bash
# Search with context
neuracode search "user authentication" --limit 20

# Export results as JSON
neuracode search "database connection" --format json > results.json
```

### Impact Analysis

```bash
# Analyze before refactoring
neuracode impact "UserService"

# Check test coverage impact
neuracode impact src/services/user.ts
```

### Context Prediction

```bash
# Get context for bug fix
neuracode predict "fix null pointer exception in login"

# Get context for new feature
neuracode predict "add user profile page"
```

### Multi-Modal Understanding

```bash
# Understand architecture diagram
neuracode understand docs/architecture.png

# Parse flowchart
neuracode understand docs/user-flow.png

# Read whiteboard photo
neuracode understand photos/design-session.jpg
```
