# NeuraCode Architecture

## Overview

NeuraCode is built on a modular architecture with Rust for performance-critical components and Python for AI/ML capabilities.

## Core Components

### 1. Code Brain (`code_brain.rs`)

The Code Brain is responsible for deep codebase understanding.

**Key Data Structures:**
- `CodeGraph`: In-memory representation of the codebase
- `CodeNode`: Functions, classes, modules, files
- `CodeEdge`: Relationships between nodes

**Capabilities:**
- AST parsing with tree-sitter
- Semantic search
- Impact analysis
- Architecture detection
- Hotspot identification

### 2. Predict Engine (`predict_engine.rs`)

The Predict Engine anticipates what context agents will need.

**Key Components:**
- `TaskClassifier`: Classifies tasks into categories
- `ContextPredictor`: Predicts required context
- `PrefetchScheduler`: Pre-loads context

**Task Types:**
- Bug Fix
- Refactor
- New Feature
- Code Review
- Performance Optimization
- Security Fix
- Documentation
- Testing
- Architecture

### 3. Learn Engine (`learn_engine.rs`)

The Learn Engine enables continuous improvement.

**Learned Information:**
- Code style
- Naming conventions
- Error handling patterns
- Testing patterns
- User preferences
- Project context

### 4. Multi-Modal Engine (`multi_modal.rs`)

Understands visual content.

**Supported Types:**
- Architecture diagrams
- Flowcharts
- Sequence diagrams
- Class diagrams
- Whiteboard photos
- Screenshots

### 5. Collaborative Reasoning (`collab_reasoning.rs`)

Enables multi-agent collaboration.

**Features:**
- Task decomposition
- Parallel execution
- Knowledge fusion
- Conflict resolution

### 6. Multi-Agent Support (`multi_agent.rs`)

Unified interface for all AI agents.

**Supported Agents:**
- Claude Code
- Cursor
- Codex
- Gemini CLI
- OpenCode
- GitHub Copilot
- Windsurf
- Cline
- Aider
- Continue

## Data Flow

```
User Query
    ↓
Task Classification
    ↓
Context Prediction
    ↓
Code Graph Query
    ↓
Knowledge Fusion
    ↓
Response
```

## Storage

### SQLite Database

- `codebase.db`: Code graph and metadata
- `learning.db`: Learned patterns and knowledge

### Cache

- In-memory graph for fast queries
- LRU cache for search results

## Performance

- **Index Speed**: ~1ms per file
- **Query Latency**: < 10ms
- **Memory Usage**: ~100MB for 10k files
