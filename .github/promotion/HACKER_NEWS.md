# Hacker News Show HN 模板

## 标题

Show HN: NeuraCode – An open-source "second brain" for AI coding assistants

## 正文

Hi HN,

I built NeuraCode because I was frustrated with how little AI coding assistants understand about my projects.

**The Problem:**
- Claude Code, Cursor, Codex don't understand your codebase structure
- You have to explain the same things every session
- They make changes without knowing the impact
- Context is lost when you close the terminal

**The Solution:**
NeuraCode gives AI assistants a persistent "second brain" that:
- Builds a knowledge graph of your entire codebase
- Predicts what context you need before you ask
- Learns your coding style over time
- Works with 10+ AI agents

**How it works:**
1. Index your codebase (1ms per file)
2. Build a code graph with functions, classes, dependencies
3. When you ask your AI something, NeuraCode provides relevant context
4. AI makes better decisions with full project understanding

**Tech:**
- Core: Rust (12,500+ lines)
- AI/ML: Python
- AST: tree-sitter (40+ languages)
- Storage: SQLite + Knowledge Graph

**Try it:**
```
curl -fsSL https://raw.githubusercontent.com/kevindurant735rocket-creator/neuracode/main/install.sh | bash
cd your-project
neuracode init && neuracode index
neuracode install --all
```

GitHub: https://github.com/kevindurant735rocket-creator/neuracode

Happy to answer questions!
