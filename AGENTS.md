# Repository Guidelines

## Project Structure & Module Organization
Core library code sits in `src/`. `src/lib.rs` exposes the public API, while `src/main.rs` hosts the CLI entry point. YARA integration utilities live under `src/testing/` and share fixtures with `test_data/`. Integration coverage extends through `tests/`, with validation scenarios in `test_validation/` plus example usage in `examples/`. Benchmarks reside in `benches/`; `docs/` captures explainer material and change logs support release notes.

## Build, Test & Development Commands
Use `cargo build` for quick checks and `cargo build --release` for optimized binaries. `cargo test --all-features` runs unit and integration suites; `cargo run -- run-tests` exercises the YARA validation flow end-to-end. Make targets orchestrate CI parity: `make all` runs fmt, lint, audit, deny, feature, test, docs, and example builds, while `make dev-setup` installs rustfmt, clippy, cargo-audit, cargo-deny, and cargo-llvm-cov. Coverage snapshots come from `cargo llvm-cov --workspace --html`.

## Coding Style & Naming Conventions
Code is auto-formatted with `cargo fmt`; run `cargo fmt --all` before committing. Favor four-space indentation, snake_case functions, SCREAMING_SNAKE_CASE constants, and PascalCase types. Keep modules small and re-export through `src/testing/mod.rs` when adding new test utilities. Enforce lint cleanliness: `cargo clippy --all-targets -- -D warnings` and `cargo deny check` must pass. Follow `deny.toml` and `clippy.toml` guidance rather than ad-hoc overrides.

## Testing Guidelines
Unit tests sit near the code they exercise; integration flows belong in `tests/` with function names mirroring the feature under test (`test_generate_batch_basic`). Add fixtures under `test_data/` or `test_validation/` and reference them with relative paths. Provide coverage evidence via `cargo llvm-cov --html` when touching core APIs, and include regression cases for any bug fix that affects YARA parsing or batch orchestration.

## Commit & Pull Request Guidelines
Commit history uses Conventional Commit semantics (`fix:`, `refactor:`, `chore:`). Keep messages imperative and map to a single logical change. Pull requests should summarize scope, list validation commands (e.g., `make all` output), link relevant issues, and attach CLI output or screenshots when behavior changes. Document any new environment variables or data files and request review before merging.

## Environment & Security Notes
Set `OPENAI_API_KEY` locally; never check secrets into the repo. Validate new dependencies with `cargo audit` and `cargo deny`, and capture rationale in the PR if exceptions are needed. When working with YARA examples, avoid embedding real malware samples—use synthetic or redacted fixtures stored under `test_data/`.
