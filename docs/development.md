# NeuraCode Development Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Structure](#project-structure)
3. [Building](#building)
4. [Testing](#testing)
5. [Adding Language Support](#adding-language-support)
6. [Adding Agent Support](#adding-agent-support)
7. [Contributing](#contributing)

## Getting Started

### Prerequisites

- Rust 1.70+
- Python 3.10+
- Cargo
- Git

### Clone and Setup

```bash
git clone https://github.com/neuracode/neuracode.git
cd neuracode

# Install Python dependencies
pip install -e python/

# Build the project
cargo build
```

## Project Structure

```
neuracode/
├── crates/
│   ├── neuracode-core/       # Core Rust library
│   │   ├── src/
│   │   │   ├── lib.rs        # Main library entry
│   │   │   ├── code_brain.rs # Code understanding
│   │   │   ├── predict_engine.rs
│   │   │   ├── learn_engine.rs
│   │   │   ├── multi_modal.rs
│   │   │   ├── collab_reasoning.rs
│   │   │   ├── multi_agent.rs
│   │   │   ├── types.rs      # Type definitions
│   │   │   ├── error.rs      # Error types
│   │   │   └── utils.rs      # Utilities
│   │   └── Cargo.toml
│   │
│   ├── neuracode-cli/        # CLI tool
│   │   ├── src/
│   │   │   ├── main.rs       # CLI entry point
│   │   │   └── commands/     # CLI commands
│   │   └── Cargo.toml
│   │
│   └── neuracode-mcp/        # MCP server
│       ├── src/
│       │   ├── main.rs
│       │   ├── server.rs
│       │   └── tools.rs
│       └── Cargo.toml
│
├── python/
│   └── neuracode/            # Python AI modules
│       ├── models/           # ML models
│       │   ├── embeddings.py
│       │   ├── classifier.py
│       │   └── predictor.py
│       ├── multimodal/       # Multi-modal understanding
│       │   ├── image.py
│       │   └── diagram.py
│       └── utils/            # Utilities
│
├── skills/                   # Agent skills
│   ├── neuracode/
│   │   └── SKILL.md
│   └── install/              # Installation scripts
│
├── docs/                     # Documentation
├── tests/                    # Tests
└── scripts/                  # Build/Release scripts
```

## Building

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Build Python Module

```bash
cd python/
maturin develop
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Tests

```bash
# Test code brain
cargo test code_brain

# Test predict engine
cargo test predict_engine

# Test with output
cargo test -- --nocapture
```

### Python Tests

```bash
cd python/
pytest
```

## Adding Language Support

### 1. Add Language to Types

Edit `crates/neuracode-core/src/types.rs`:

```rust
pub enum Language {
    // ... existing languages
    YourLanguage,
}
```

### 2. Add Tree-sitter Parser

Edit `crates/neuracode-core/src/code_brain.rs`:

```rust
fn init_parsers(languages: &[Language]) -> Result<DashMap<Language, Parser>> {
    // ... existing parsers
    if languages.contains(&Language::YourLanguage) {
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_yourlanguage::language())?;
        parsers.insert(Language::YourLanguage, parser);
    }
    // ...
}
```

### 3. Add Node Kind Mapping

```rust
fn map_node_kind(&self, kind: &str, language: Language) -> Option<NodeKind> {
    match language {
        // ... existing languages
        Language::YourLanguage => match kind {
            "function_definition" => Some(NodeKind::Function),
            // ... map your language's node kinds
            _ => None,
        },
        _ => None,
    }
}
```

### 4. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_your_language_parsing() {
        // Add test code
    }
}
```

## Adding Agent Support

### 1. Add Agent Type

Edit `crates/neuracode-core/src/types.rs`:

```rust
pub enum AgentType {
    // ... existing agents
    YourAgent,
}
```

### 2. Add Agent Configuration

Edit `crates/neuracode-core/src/multi_agent.rs`:

```rust
fn init_agent_configs(configs: &mut HashMap<AgentType, AgentConfig>) {
    // ... existing configs
    configs.insert(AgentType::YourAgent, AgentConfig {
        agent_type: AgentType::YourAgent,
        config_path: home.join(".youragent").join("config.json"),
        mcp_server_name: "neuracode".to_string(),
        instructions_path: Some(home.join(".youragent").join("INSTRUCTIONS.md")),
        skills_path: None,
    });
}
```

### 3. Add Installation Logic

```rust
async fn install_mcp_server(&self, config: &AgentConfig) -> Result<()> {
    match config.agent_type {
        // ... existing agents
        AgentType::YourAgent => {
            // Custom installation logic
        }
        _ => {
            // Default installation
        }
    }
}
```

## Contributing

### Code Style

- Use `rustfmt` for Rust code
- Use `black` for Python code
- Follow the existing code style

### Commit Messages

Follow conventional commits:

```
feat: add support for NewLanguage
fix: resolve issue with impact analysis
docs: update usage guide
test: add tests for predict engine
```

### Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Update documentation
6. Submit a pull request

### Reporting Issues

Please include:
- NeuraCode version
- Operating system
- Steps to reproduce
- Expected behavior
- Actual behavior
