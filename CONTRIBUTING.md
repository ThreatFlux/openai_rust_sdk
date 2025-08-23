# Contributing to OpenAI Rust SDK

Thank you for your interest in contributing to the OpenAI Rust SDK! We welcome contributions from the community.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Release Process](#release-process)

## Code of Conduct

Please be respectful and constructive in all interactions. We aim to maintain a welcoming and inclusive environment for all contributors.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/openai_rust_sdk.git
   cd openai_rust_sdk
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/threatflux/openai_rust_sdk.git
   ```
4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Process

### Prerequisites

- Rust 1.82 or later
- Cargo and standard Rust toolchain
- Optional: Docker for testing containerized builds

### Building

```bash
# Standard build
cargo build --all-features

# Release build
cargo build --release --all-features

# Quick development cycle
make dev
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Run full CI-like checks
make all

# Or run individually:
cargo fmt           # Format code
cargo clippy        # Lint
cargo test         # Run tests
cargo audit        # Security audit
```

## Commit Guidelines

⚠️ **IMPORTANT**: This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning and releases.

Please read our [Commit Convention Guide](.github/commit-convention.md) for detailed information.

### Quick Reference

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types that trigger releases:**
- `feat:` - New feature (minor version bump)
- `fix:` - Bug fix (patch version bump)
- `feat!:` or `BREAKING CHANGE:` - Breaking change (major version bump)

**Other types (no release):**
- `docs:` - Documentation
- `style:` - Formatting
- `refactor:` - Code restructuring
- `perf:` - Performance improvements
- `test:` - Tests
- `chore:` - Maintenance
- `ci:` - CI/CD changes

### Examples

```bash
# Feature (triggers minor release)
git commit -m "feat: add support for GPT-5 models"

# Bug fix (triggers patch release)
git commit -m "fix: resolve memory leak in streaming responses"

# Breaking change (triggers major release)
git commit -m "feat!: redesign API client initialization

BREAKING CHANGE: Client::new() now requires ApiConfig parameter"
```

## Pull Request Process

1. **Update your fork:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Make your changes:**
   - Write clear, concise code
   - Add tests for new functionality
   - Update documentation as needed
   - Follow existing code style

3. **Test thoroughly:**
   ```bash
   make test          # Run all tests
   make test-openai   # Test with OpenAI API (requires API key)
   ```

4. **Create PR:**
   - Use a descriptive title following commit conventions
   - Fill out the PR template
   - Link related issues
   - Ensure all CI checks pass

5. **PR Review:**
   - Address reviewer feedback
   - Keep PR focused and atomic
   - Rebase if needed to resolve conflicts

## Testing

### Unit Tests
```bash
cargo test --all-features
```

### Integration Tests
```bash
# Requires OPENAI_API_KEY environment variable
export OPENAI_API_KEY=your_key_here
cargo test --all-features -- --ignored
```

### Coverage
```bash
make coverage      # Generate coverage report
make coverage-open # Open HTML report
```

### Examples
Test examples to ensure they compile and run:
```bash
cargo run --example chat_completion
cargo run --example streaming_chat
```

## Release Process

### Automatic Releases

This project uses automated releases triggered by conventional commits:

1. **Push commits** with proper conventional commit messages
2. **CI/CD runs** automatically on push to main
3. **Auto-release workflow** triggers when all checks pass
4. **Version bumped** based on commit types since last release
5. **Release created** with:
   - Updated version in Cargo.toml
   - Generated CHANGELOG.md
   - Git tag
   - GitHub Release with artifacts
   - Published to crates.io (if configured)

### Manual Release

To trigger a release manually:

1. Go to Actions → Auto Release workflow
2. Click "Run workflow"
3. Select version bump type (patch/minor/major)
4. Click "Run workflow"

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

## Project Structure

```
openai_rust_sdk/
├── src/
│   ├── api/           # API client implementations
│   ├── models/        # Request/response models
│   ├── builders/      # Builder patterns
│   ├── testing/       # Test utilities and YARA validation
│   └── client.rs      # Main client
├── examples/          # Usage examples
├── tests/            # Integration tests
├── benches/          # Benchmarks
└── .github/
    └── workflows/    # CI/CD pipelines
```

## Getting Help

- Check existing [issues](https://github.com/threatflux/openai_rust_sdk/issues)
- Read the [documentation](https://docs.rs/openai_rust_sdk)
- Ask in [discussions](https://github.com/threatflux/openai_rust_sdk/discussions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.