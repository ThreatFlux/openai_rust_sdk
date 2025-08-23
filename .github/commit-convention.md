# Commit Message Convention

This project uses [Conventional Commits](https://www.conventionalcommits.org/) for automatic versioning and changelog generation.

## Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Types

- **feat**: A new feature (triggers MINOR version bump)
- **fix**: A bug fix (triggers PATCH version bump)
- **docs**: Documentation only changes
- **style**: Changes that don't affect code meaning (formatting, etc.)
- **refactor**: Code change that neither fixes a bug nor adds a feature
- **perf**: Performance improvements
- **test**: Adding or updating tests
- **chore**: Changes to build process or auxiliary tools
- **ci**: Changes to CI configuration files and scripts

## Breaking Changes

Add `BREAKING CHANGE:` in the footer or `!` after the type to trigger a MAJOR version bump:

```
feat!: remove deprecated API endpoints

BREAKING CHANGE: The /api/v1/* endpoints have been removed in favor of /api/v2/*
```

## Examples

### Feature (Minor version bump)
```
feat: add GPT-5 model support

- Implement reasoning_effort parameter
- Add new model constants
- Update documentation
```

### Bug Fix (Patch version bump)
```
fix: resolve OpenSSL dependency issues for cross-compilation

Switch to rustls-tls to eliminate OpenSSL requirements
```

### Breaking Change (Major version bump)
```
feat!: restructure API client initialization

BREAKING CHANGE: Client::new() now requires explicit API key parameter
instead of reading from environment variable
```

## Automatic Release Triggers

The auto-release workflow will create a new release when:

1. All CI checks pass (CI, Quality, Security workflows)
2. There are conventional commits since the last release:
   - Any `feat:` commits → Minor release
   - Any `fix:` commits → Patch release  
   - Any `BREAKING CHANGE:` → Major release
3. Manual trigger via workflow dispatch

## Release Process

1. Push commits with conventional commit messages
2. Wait for CI/CD to pass
3. Auto-release workflow triggers automatically
4. Version is bumped based on commit types
5. CHANGELOG.md is updated
6. Git tag is created
7. GitHub Release is published
8. Release artifacts are built and attached