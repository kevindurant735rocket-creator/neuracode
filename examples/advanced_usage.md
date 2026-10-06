# NeuraCode Advanced Usage

This guide covers advanced NeuraCode features and workflows.

## Table of Contents

1. [Custom Configuration](#custom-configuration)
2. [Semantic Search](#semantic-search)
3. [Impact Analysis](#impact-analysis)
4. [Context Prediction](#context-prediction)
5. [Multi-Modal Understanding](#multi-modal-understanding)
6. [Code Export](#code-export)
7. [Watching for Changes](#watching-for-changes)
8. [Comparing Codebases](#comparing-codebases)

## Custom Configuration

Edit `.neuracode/config.toml`:

```toml
# Cache settings
cache_size_mb = 1024

# Feature flags
enable_prediction = true
enable_learning = true
enable_multimodal = true

# Supported languages
languages = ["rust", "typescript", "python"]

# Ignore patterns
ignore_patterns = [
    "node_modules",
    "dist",
    "build",
    "*.test.ts",
]

# Maximum file size (1MB)
max_file_size = 1048576
```

## Semantic Search

### Basic Search

```bash
neuracode search "user authentication"
```

### Advanced Search

```bash
# Limit results
neuracode search "database" --limit 20

# JSON output
neuracode search "api endpoints" --format json

# Search with context
neuracode search "error handling" --context 5
```

## Impact Analysis

Before refactoring, always check the impact:

```bash
# Analyze a file
neuracode impact src/services/user.ts

# Analyze a function
neuracode impact "authenticateUser"

# JSON output for automation
neuracode impact src/auth.ts --format json
```

## Context Prediction

Let NeuraCode prepare context for your tasks:

```bash
# Bug fix
neuracode predict "fix null pointer in login"

# New feature
neuracode predict "add user profile page"

# Refactoring
neuracode predict "refactor authentication module"
```

## Multi-Modal Understanding

### Architecture Diagrams

```bash
neuracode understand docs/architecture.png
```

### Flowcharts

```bash
neuracode understand docs/user-flow.png
```

### Whiteboard Photos

```bash
neuracode understand photos/design-session.jpg
```

## Code Export

### Export as JSON

```bash
neuracode export graph.json --format json
```

### Export as Graphviz DOT

```bash
neuracode export graph.dot --format dot
```

### Export as Mermaid

```bash
neuracode export graph.mmd --format mermaid
```

## Watching for Changes

Auto-index when files change:

```bash
neuracode watch
```

## Comparing Codebases

Compare two versions of your code:

```bash
neuracode compare ./old-version ./new-version

# JSON output
neuracode compare ./v1.0 ./v2.0 --format json
```

## Automation Examples

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Check impact of changes
neuracode impact $(git diff --name-only)

# Run analysis
neuracode analyze --format json > analysis.json
```

### CI/CD Integration

```yaml
# .github/workflows/analysis.yml
name: Code Analysis
on: [push, pull_request]

jobs:
  analyze:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install NeuraCode
        run: curl -fsSL https://raw.githubusercontent.com/neuracode/neuracode/main/install.sh | bash
      - name: Index codebase
        run: neuracode index
      - name: Run analysis
        run: neuracode analyze --format json > analysis.json
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: analysis
          path: analysis.json
```
