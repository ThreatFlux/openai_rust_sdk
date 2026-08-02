# Contributing to OpenAI Rust SDK

Thank you for helping improve the project. Contributions should be focused, tested, and honest
about the OpenAI API behavior they support. Please keep discussion respectful and constructive.

Security vulnerabilities must follow the private process in [SECURITY.md](SECURITY.md); do not
open a public issue for them.

## Set up a development checkout

The minimum supported Rust version is **1.97.1**. The checked-in `rust-toolchain.toml` selects the
project toolchain automatically when Rustup is installed. Documentation checks additionally use
Python 3.11 or newer.

```bash
git clone https://github.com/YOUR-USER/openai_rust_sdk.git
cd openai_rust_sdk
git remote add upstream https://github.com/ThreatFlux/openai_rust_sdk.git
git switch -c your-focused-branch
```

Install the optional development tools used by the full Makefile workflow:

```bash
make dev-setup
```

Normal builds and tests do not need an OpenAI API key. Never commit credentials, `.env` files,
generated Batch inputs containing sensitive data, or live API responses with private content.

## Development loop

Use the smallest check that covers your change while iterating:

```bash
cargo check --all-targets --all-features
cargo test --all-features
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

Before opening a pull request, run the repository checks:

```bash
make ci
python3 scripts/check_docs.py
```

`make ci` uses additional tools installed by `make dev-setup`, including `cargo-audit` and
`cargo-deny`. It can require network access to refresh advisory data.

For a documentation-only change, at minimum run:

```bash
python3 scripts/check_docs.py
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
```

The live integration test is environment-gated rather than marked `#[ignore]`. If
`OPENAI_API_KEY` is present, ordinary `cargo test --all-features` and `make ci` runs can make live
requests and incur usage charges. Unset the variable for an offline test run. To run the live test
intentionally:

```bash
export OPENAI_API_KEY="your_api_key_here"
make test-openai
```

Use a restricted test project and review each live test before running it. Most contributions
should be verifiable with unit tests, fixtures, and mock HTTP servers.

## Adding or changing API support

An API contribution should normally include:

- typed request and response models, with optional fields matching the wire format;
- the client operation and error behavior;
- serialization and mocked HTTP tests for success and representative failures;
- a compile-tested example when users need a new integration pattern; and
- an update to `docs/api-coverage.md` and any affected configuration guidance.

Do not describe an area as fully supported solely because one endpoint exists. Call out missing
operations, untyped events, preview behavior, and deprecated surfaces explicitly.

Feature-gated changes should compile both without default features and with all features:

```bash
cargo check --no-default-features
cargo check --all-features
```

## Commits and pull requests

This repository uses [Conventional Commits](.github/commit-convention.md). Examples:

```text
feat(responses): support response compaction
fix(streaming): preserve API error payloads
docs: clarify custom base URL handling
```

Keep a pull request focused on one concern. Its description should explain the user-visible
behavior, testing performed, compatibility impact, and any API gaps left intentionally. Link
related issues and update the changelog when maintainers request it.

Before requesting review, confirm that:

- formatting, linting, relevant tests, and docs checks pass;
- public API additions have useful rustdoc comments;
- examples use current, configurable models and do not embed secrets;
- generated files and unrelated formatting changes are excluded; and
- the README and coverage matrix still make claims the implementation supports.

Project maintainers handle versioning, release notes, tags, and crates.io publication after a
change is merged.

## Getting help

Use [GitHub Issues](https://github.com/ThreatFlux/openai_rust_sdk/issues) for focused,
reproducible bugs, design questions, or feature proposals.
