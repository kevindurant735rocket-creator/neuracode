# Contributing to NeuraCode

Thank you for your interest in contributing to NeuraCode! This document provides guidelines and instructions for contributing.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Workflow](#development-workflow)
- [Style Guidelines](#style-guidelines)
- [Commit Messages](#commit-messages)
- [Pull Requests](#pull-requests)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior.

## Getting Started

### Prerequisites

- Rust 1.70+
- Python 3.10+
- Git

### Setup

1. Fork the repository
2. Clone your fork
3. Install dependencies

```bash
git clone https://github.com/yourusername/neuracode.git
cd neuracode
pip install -e python/
```

## How to Contribute

### Reporting Bugs

Before creating a bug report, please check existing issues. When creating a bug report, include:

- **Title**: Clear and descriptive
- **Description**: Detailed description of the bug
- **Steps to Reproduce**: Step-by-step instructions
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Rust version, Python version
- **Additional Context**: Screenshots, logs, etc.

### Suggesting Enhancements

Enhancement suggestions are welcome. When suggesting:

- **Title**: Clear and descriptive
- **Description**: Detailed description of the enhancement
- **Use Case**: Why this enhancement would be useful
- **Implementation**: Ideas for implementation (optional)

### Contributing Code

1. Find an issue to work on
2. Comment on the issue to claim it
3. Create a feature branch
4. Make your changes
5. Add tests
6. Update documentation
7. Submit a pull request

## Development Workflow

### Creating a Branch

```bash
git checkout -b feature/your-feature-name
```

or

```bash
git checkout -b fix/your-bug-fix
```

### Making Changes

1. Make your changes
2. Add tests for new functionality
3. Update documentation
4. Run tests to ensure everything works

### Running Tests

```bash
# Run all tests
cargo test

# Run specific tests
cargo test code_brain

# Run Python tests
cd python && pytest
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Style Guidelines

### Rust Code

- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `rustfmt` to format code
- Use `clippy` for linting
- Add documentation comments for public APIs
- Write unit tests for new functionality

### Python Code

- Follow [PEP 8](https://peps.python.org/pep-0008/)
- Use `black` for formatting
- Use `ruff` for linting
- Add docstrings for public APIs
- Write unit tests for new functionality

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build process or auxiliary tool changes

Examples:
```
feat(code_brain): add support for TypeScript generics

fix(predict_engine): resolve issue with context prediction

docs: update usage guide with new examples
```

## Pull Requests

### Before Submitting

1. Ensure all tests pass
2. Update documentation
3. Add tests for new functionality
4. Run `cargo fmt` and `cargo clippy`
5. Run `black` and `ruff` for Python code

### PR Description

Include:
- **Title**: Clear and descriptive
- **Description**: What changes were made and why
- **Testing**: How the changes were tested
- **Screenshots**: If applicable
- **Related Issues**: Link to related issues

### Review Process

1. A maintainer will review your PR
2. You may be asked to make changes
3. Once approved, your PR will be merged

## Questions?

If you have questions, please:
1. Check the [documentation](docs/)
2. Search [existing issues](https://github.com/neuracode/neuracode/issues)
3. Ask in [Discussions](https://github.com/neuracode/neuracode/discussions)
4. Join our [Discord](https://discord.gg/neuracode)

Thank you for contributing to NeuraCode!
