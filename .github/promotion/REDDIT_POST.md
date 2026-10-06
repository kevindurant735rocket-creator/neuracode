# Reddit 发帖模板

## r/programming

**标题**: [Project] NeuraCode - An open-source "second brain" for AI coding assistants

**正文**:

Hi r/programming,

I've been working on NeuraCode, an open-source tool that gives AI coding assistants a "second brain" to truly understand your codebase.

## The Problem

If you use AI coding assistants (Claude Code, Cursor, Codex, etc.), you've probably experienced:
- AI doesn't understand your project structure
- You have to explain the same things repeatedly
- AI makes changes without knowing the impact
- Context is lost between sessions

## The Solution

NeuraCode enhances AI assistants with:

🧠 **Deep Codebase Understanding**
- Builds a complete knowledge graph of your code
- Understands dependencies and relationships
- Identifies hotspots and critical paths

🔮 **Predictive Context**
- Anticipates what context you need
- Prepares relevant files before you ask
- Suggests related code and tests

📚 **Continuous Learning**
- Learns your coding style
- Remembers your preferences
- Gets smarter with every interaction

🌐 **Universal Support**
- Works with 10+ AI agents
- One-line installation
- 100% local, privacy-first

## Tech Stack

- **Core**: Rust (12,500+ lines)
- **AI/ML**: Python
- **AST Parsing**: tree-sitter (40+ languages)
- **Storage**: SQLite + Knowledge Graph

## Quick Start

```bash
# Install
curl -fsSL https://raw.githubusercontent.com/kevindurant735rocket-creator/neuracode/main/install.sh | bash

# Initialize
cd your-project
neuracode init

# Index your codebase
neuracode index

# Install for your AI agent
neuracode install --all
```

## GitHub

https://github.com/kevindurant735rocket-creator/neuracode

I'd love to hear your feedback and answer any questions!

---

**Edit**: Thanks for the amazing response! To answer some common questions:

1. **Why Rust?** Performance and safety. Indexing 10k files takes ~1ms per file.

2. **Is it private?** Yes! Everything runs locally. Your code never leaves your machine.

3. **Which AI agents are supported?** Claude Code, Cursor, Codex, Gemini CLI, OpenCode, GitHub Copilot, Windsurf, Cline, Aider, Continue.

4. **How is it different from RAG?** NeuraCode builds a persistent knowledge graph, not just vector embeddings. It understands code structure, not just content.
