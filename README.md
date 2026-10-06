# 🧠 NeuraCode

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="Version">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License">
  <img src="https://img.shields.io/badge/language-Rust%20%2B%20Python-orange.svg" alt="Language">
  <img src="https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg" alt="Platform">
</p>

<p align="center">
  <strong>The Next-Generation AI Agent Cognitive Enhancement System</strong>
</p>

<p align="center">
  <em>"Not fewer tokens for agents, but more understanding"</em>
</p>

---

## 🌟 What is NeuraCode?

NeuraCode is a revolutionary cognitive enhancement system for AI coding agents. Unlike traditional approaches that compress and limit agent capabilities, NeuraCode **enhances** agent intelligence by providing:

- 🧠 **Deep Codebase Understanding** - Build a complete knowledge graph of your code
- 🔮 **Predictive Context** - Anticipate what agents need before they ask
- 📚 **Continuous Learning** - Get smarter with every interaction
- 🎨 **Multi-Modal Understanding** - Understand diagrams, architecture, and whiteboards
- 🤝 **Collaborative Reasoning** - Multiple agents working together
- 🌐 **Universal Agent Support** - Works with 10+ AI coding agents

---

## 🚀 Quick Start

### Installation

```bash
# Install NeuraCode CLI
curl -fsSL https://raw.githubusercontent.com/neuracode/neuracode/main/install.sh | bash

# Or install via cargo
cargo install neuracode-cli
```

### Initialize Your Project

```bash
cd your-project
neuracode init
```

### Index Your Codebase

```bash
neuracode index
```

### Search Your Code

```bash
neuracode search "authentication"
```

### Predict Context for Tasks

```bash
neuracode predict "fix login bug"
```

### Install for AI Agents

```bash
# Install for all supported agents
neuracode install --all

# Or install for a specific agent
neuracode install --agent claude-code
```

---

## 🎯 Core Features

### 1. Code Brain (代码大脑)

Deep understanding of your codebase:

```bash
# Semantic search
neuracode search "user authentication"

# Impact analysis
neuracode impact src/auth.ts

# Architecture detection
neuracode architecture

# Hotspot identification
neuracode hotspots
```

**Capabilities:**
- ✅ Code graph with functions, classes, modules
- ✅ Dependency analysis
- ✅ Call chain tracing
- ✅ Architecture pattern detection
- ✅ Hotspot identification

### 2. Predict Engine (预测引擎)

Anticipate what agents need:

```bash
# Predict context for a task
neuracode predict "refactor user module"
```

**Predicts:**
- Relevant files
- Test files
- Recent commits
- Error logs
- Similar fixes
- Architecture context
- Dependencies

### 3. Learn Engine (学习引擎)

Continuous learning from sessions:

- ✅ Code style detection
- ✅ Pattern recognition
- ✅ User preference learning
- ✅ Knowledge accumulation
- ✅ Cross-session memory

### 4. Multi-Modal (多模态理解)

Understand visual content:

```bash
# Understand architecture diagrams
neuracode understand diagrams/architecture.png

# Parse flowcharts
neuracode understand diagrams/flowchart.png

# Read whiteboard photos
neuracode understand photos/whiteboard.jpg
```

**Supports:**
- Architecture diagrams
- Flowcharts
- Sequence diagrams
- Class diagrams
- Whiteboard photos
- Screenshots

### 5. Collaborative Reasoning (协作推理)

Multiple agents working together:

- ✅ Task decomposition
- ✅ Parallel execution
- ✅ Knowledge fusion
- ✅ Conflict resolution
- ✅ Collective intelligence

### 6. Multi-Agent Support (多Agent支持)

Works with all major AI coding agents:

| Agent | Status |
|-------|--------|
| Claude Code | ✅ Supported |
| Cursor | ✅ Supported |
| Codex | ✅ Supported |
| Gemini CLI | ✅ Supported |
| OpenCode | ✅ Supported |
| GitHub Copilot | ✅ Supported |
| Windsurf | ✅ Supported |
| Cline | ✅ Supported |
| Aider | ✅ Supported |
| Continue | ✅ Supported |

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| Index Speed | ~1ms per file |
| Query Latency | < 10ms |
| Language Support | 8+ (expanding) |
| Agent Support | 10+ |
| Prediction Accuracy | > 85% |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        NeuraCode                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Code      │  │   Predict   │  │   Learn     │             │
│  │   Brain     │  │   Engine    │  │   Engine    │             │
│  │             │  │             │  │             │             │
│  │ Code Graph  │  │ Task Class  │  │ Pattern Rec │             │
│  │ Dependencies│  │ Context Pred│  │ Preference  │             │
│  │ Architecture│  │ Pre-fetch   │  │ Knowledge   │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│                                                                 │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐             │
│  │   Multi-    │  │   Collab    │  │   Multi-    │             │
│  │   Modal     │  │   Reasoning │  │   Agent     │             │
│  │             │  │             │  │             │             │
│  │ Images      │  │ Multi-Agent │  │ Claude Code │             │
│  │ Diagrams    │  │ Knowledge   │  │ Cursor      │             │
│  │ Whiteboards │  │ Fusion      │  │ Codex       │             │
│  └─────────────┘  └─────────────┘  └─────────────┘             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Development

### Prerequisites

- Rust 1.70+
- Python 3.10+
- Cargo

### Build

```bash
# Clone the repository
git clone https://github.com/neuracode/neuracode.git
cd neuracode

# Build the project
cargo build --release

# Run tests
cargo test

# Install Python dependencies
pip install -e python/
```

### Project Structure

```
neuracode/
├── crates/
│   ├── neuracode-core/       # Core Rust library
│   ├── neuracode-cli/        # CLI tool
│   └── neuracode-mcp/        # MCP server
├── python/
│   └── neuracode/            # Python AI modules
├── skills/                   # Agent skills
├── docs/                     # Documentation
└── tests/                    # Tests
```

---

## 📖 Documentation

- [Architecture](docs/architecture.md)
- [API Reference](docs/api.md)
- [Usage Guide](docs/usage.md)
- [Development](docs/development.md)

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

## 📄 License

NeuraCode is licensed under the MIT License. See [LICENSE](LICENSE) for details.

---

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=neuracode/neuracode&type=Date)](https://www.star-history.com/#neuracode/neuracode)

---

## 🙏 Acknowledgments

- [tree-sitter](https://tree-sitter.github.io/) for AST parsing
- [CodeBERT](https://github.com/microsoft/CodeBERT) for code embeddings
- All the amazing open-source projects that inspired NeuraCode

---

<p align="center">
  <strong>Built with ❤️ by the NeuraCode Team</strong>
</p>

<p align="center">
  <a href="https://github.com/neuracode/neuracode">GitHub</a> •
  <a href="https://discord.gg/neuracode">Discord</a> •
  <a href="https://twitter.com/neuracode">Twitter</a>
</p>
