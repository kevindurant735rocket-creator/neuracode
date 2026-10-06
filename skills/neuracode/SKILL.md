# NeuraCode Skill

## Description

NeuraCode provides enhanced code intelligence for AI agents. It builds a deep understanding of your codebase and provides predictive context for tasks.

## Trigger

Use this skill when:
- Searching for code
- Analyzing code structure
- Understanding dependencies
- Planning refactoring
- Reviewing code changes

## Commands

### Search the Codebase

```
/neuracode search <query>
```

Semantic search across your entire codebase.

**Examples:**
- `/neuracode search authentication`
- `/neuracode search "user login"`
- `/neuracode search database connection`

### Predict Context

```
/neuracode predict <task>
```

Predict what context is needed for a task.

**Examples:**
- `/neuracode predict "fix login bug"`
- `/neuracode predict "refactor user module"`
- `/neuracode predict "add new feature"`

### Impact Analysis

```
/neuracode impact <target>
```

Analyze the impact of changing a file or function.

**Examples:**
- `/neuracode impact src/auth.ts`
- `/neuracode impact "UserService"`

### Architecture

```
/neuracode architecture
```

Show the detected architecture pattern.

### Hotspots

```
/neuracode hotspots
```

Identify the most critical parts of your codebase.

### Understand Images

```
/neuracode understand <image>
```

Understand architecture diagrams, flowcharts, and whiteboard photos.

**Examples:**
- `/neuracode understand diagrams/architecture.png`
- `/neuracode understand photos/whiteboard.jpg`

## How It Works

NeuraCode builds a knowledge graph of your codebase by:
1. Parsing source files with tree-sitter
2. Extracting functions, classes, and modules
3. Building dependency relationships
4. Detecting architecture patterns
5. Identifying hotspots

This knowledge graph enables:
- Fast semantic search
- Accurate impact analysis
- Predictive context preparation
- Continuous learning

## Tips

1. **Index regularly** - Run `neuracode index` after major changes
2. **Use specific queries** - More specific queries give better results
3. **Check impact before refactoring** - Always analyze impact first
4. **Review architecture** - Understand the big picture before making changes
