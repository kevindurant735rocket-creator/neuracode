# NeuraCode Quickstart Guide

This guide will help you get started with NeuraCode in 5 minutes.

## Prerequisites

- Rust 1.70+
- Python 3.10+
- An AI coding assistant (Claude Code, Cursor, etc.)

## Step 1: Install NeuraCode

```bash
curl -fsSL https://raw.githubusercontent.com/neuracode/neuracode/main/install.sh | bash
```

## Step 2: Initialize Your Project

```bash
cd your-project
neuracode init
```

This creates a `.neuracode` directory with configuration.

## Step 3: Index Your Codebase

```bash
neuracode index
```

You should see output like:
```
✓ Indexing complete!
  Files indexed: 150
  Nodes created: 1,234
  Edges created: 5,678
  Duration: 234ms
  Languages: TypeScript, JavaScript
```

## Step 4: Search Your Code

```bash
neuracode search "authentication"
```

## Step 5: Install for Your AI Agent

```bash
# For Claude Code
neuracode install --agent claude-code

# For Cursor
neuracode install --agent cursor

# For all agents
neuracode install --all
```

## Step 6: Restart Your AI Agent

Restart your AI coding assistant and start using NeuraCode!

## Next Steps

- Read the [Usage Guide](../docs/usage.md)
- Explore the [Web UI](../web)
- Join our [Discord](https://discord.gg/neuracode)
