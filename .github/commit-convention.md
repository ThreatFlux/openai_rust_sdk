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

## Release proposal and publication

The pinned reusable auto-release workflow requires passing CI and Security
results for the target commit. With `create-pr: true`, it proposes version and
release-note changes in an automation-owned PR for maintainer review. Ordinary
feature merges do not directly publish a release.

Classification uses conventional commit subjects and footers: `feat` requires a
minor bump, `fix` a patch bump, and a subject `!` or line-start `BREAKING CHANGE:`
requires a major bump. Preserve these markers when squashing or rewriting a PR.
Manual workflow dispatch can select a bump explicitly.

Maintainers review the proposed version, compatibility notes, and checks before
merging the release PR. The reusable workflow then owns tags and GitHub releases.
It acts as the ThreatFlux automation GitHub App, so the tag it pushes starts
`release.yml` (binaries, SBOM, crates.io trusted publishing) and `docker.yml`
through their own tag triggers; without the App it dispatches both for the tag.
Do not add a second versioning or publishing path. A source PR must leave the
manifest version unchanged unless a maintainer requests a version change.
