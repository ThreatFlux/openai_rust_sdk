# OpenAI Rust SDK

[![Crates.io](https://img.shields.io/crates/v/openai_rust_sdk.svg)](https://crates.io/crates/openai_rust_sdk)
[![Documentation](https://docs.rs/openai_rust_sdk/badge.svg)](https://docs.rs/openai_rust_sdk)
[![CI](https://github.com/ThreatFlux/openai_rust_sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/ThreatFlux/openai_rust_sdk/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.1-blue.svg)](https://github.com/ThreatFlux/openai_rust_sdk/blob/main/Cargo.toml)
[![License](https://img.shields.io/crates/l/openai_rust_sdk.svg)](LICENSE)

An unofficial, async, type-safe Rust client for the OpenAI API, with first-class support for
Responses, typed streaming, modern tools, and a broad platform surface.

> [!NOTE]
> This is a community-maintained project. It is not affiliated with, endorsed by, or maintained
> by OpenAI.

[API documentation](https://docs.rs/openai_rust_sdk) · [Examples](examples/) ·
[API coverage](docs/api-coverage.md) · [Configuration](docs/configuration.md) ·
[Changelog](CHANGELOG.md) · [Security](SECURITY.md)

## Why this SDK?

- **Responses first:** create, stream, retrieve, list, cancel, compact, and count tokens with
  strongly typed requests and events.
- **Modern tools:** function calling, web and file search, MCP, image generation, computer use,
  shell, apply-patch, and custom tools.
- **Broad API surface:** conversations, Realtime, Batch, files, uploads, vector stores, media,
  evals, fine-tuning, moderation, and administration APIs.

The OpenAI API changes quickly. See the dated [coverage matrix](docs/api-coverage.md) for exact
support and known gaps instead of relying on an “all APIs” claim.

> [!WARNING]
> OpenAI has deprecated the Assistants API and announced shutdown on **August 26, 2026**. This
> crate retains Assistants, Threads, and Runs for migration support; start new integrations with
> the [Responses API](https://developers.openai.com/api/docs/guides/migrate-to-responses).

## Quick start

### Requirements

- Rust **1.97.1** or newer (the minimum supported Rust version, or MSRV)
- An [OpenAI API key](https://platform.openai.com/api-keys) for live requests
- Tokio with its macros and multithreaded runtime enabled

Install the latest published release:

```bash
cargo add openai_rust_sdk
cargo add tokio --features macros,rt-multi-thread
```

The default branch can be ahead of the crates.io release. To test unreleased `main` explicitly:

```bash
cargo add openai_rust_sdk --git https://github.com/ThreatFlux/openai_rust_sdk.git
cargo add tokio --features macros,rt-multi-thread
```

Set your credentials. `OPENAI_MODEL` is read by this example and can be omitted:

```bash
export OPENAI_API_KEY="your_api_key_here"
export OPENAI_MODEL="gpt-5.6-luna"
```

PowerShell users can use `$Env:OPENAI_API_KEY = "your_api_key_here"`.

Create `src/main.rs`:

<!-- BEGIN QUICKSTART -->
```rust
use openai_rust_sdk::{from_env, models::CreateResponseRequest};

const DEFAULT_MODEL: &str = "gpt-5.6-luna";

#[tokio::main]
async fn main() -> openai_rust_sdk::Result<()> {
    let client = from_env()?;
    let model = std::env::var("OPENAI_MODEL")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_MODEL.to_owned());

    let request = CreateResponseRequest::new_text(
        model,
        "Explain Rust's ownership model in one concise sentence.",
    );
    let response = client.create_response_v2(&request).await?;

    println!("{}", response.output_text());
    Ok(())
}
```
<!-- END QUICKSTART -->

Run it:

```bash
cargo run
```

The complete source is kept in [`examples/quickstart.rs`](examples/quickstart.rs) and compiled by
documentation CI.

## API coverage

This table is a summary, not a completeness claim. “Partial” means the crate supports useful
operations but does not yet mirror every endpoint or event in the current OpenAI API.

| Area | Status | Start here |
| --- | --- | --- |
| Responses and Conversations | Supported | [Quick start](examples/quickstart.rs), [Responses example](examples/responses_api.rs) |
| Typed streaming and tools | Partial | [Streaming](examples/responses_api.rs), [MCP](examples/responses_mcp_tool.rs), [response formats](examples/response_format_demo.rs) |
| Batch, Files, Uploads, and Vector Stores | Supported | [Batch](examples/batch_list_jobs.rs), [Files](examples/files_demo.rs), [Vector Stores](examples/vector_stores_demo.rs) |
| Audio, Images, and Videos | Partial | [Audio](examples/audio_demo.rs), [Images](examples/images_demo.rs) |
| Realtime | Partial | [Realtime example](examples/realtime_audio_demo.rs) |
| Evals, Fine-tuning, and Administration | Partial | [Fine-tuning](examples/fine_tuning_demo.rs) |
| Assistants, Threads, and Runs | Legacy | [Migration guidance](https://developers.openai.com/api/docs/guides/migrate-to-responses) |

See [API coverage](docs/api-coverage.md) for endpoint-level notes, lifecycle information, and
currently unsupported surfaces.

## Common examples

| Goal | Example | Command |
| --- | --- | --- |
| Make a Responses request | [`quickstart.rs`](examples/quickstart.rs) | `cargo run --example quickstart` |
| Stream typed Responses events | [`responses_api.rs`](examples/responses_api.rs) | `cargo run --example responses_api` |
| Use a remote MCP tool | [`responses_mcp_tool.rs`](examples/responses_mcp_tool.rs) | `cargo run --example responses_mcp_tool` |
| Build and validate structured-output schemas | [`response_format_demo.rs`](examples/response_format_demo.rs) | `cargo run --example response_format_demo` |
| List and inspect Batch jobs | [`batch_list_jobs.rs`](examples/batch_list_jobs.rs) | `cargo run --example batch_list_jobs` |
| Manage API files | [`files_demo.rs`](examples/files_demo.rs) | `cargo run --example files_demo` |

Examples that make network requests require `OPENAI_API_KEY`. Some examples exercise partial or
legacy APIs; read their source and the [coverage matrix](docs/api-coverage.md) before using them in
production.

## Cargo features

Optional features are disabled by default.

| Feature | What it enables |
| --- | --- |
| `default` | OpenAI API client and the core SDK surface |
| `testing` | Reserved compatibility feature; currently adds no dependencies |
| `yara` | Local YARA-X validation and CLI tooling for the optional Batch API example |
| `full` | All optional capabilities; currently enables `testing` and `yara` |

The `yara` feature supports a security-oriented
[Batch API example](docs/examples/batch-yara-x.md); it is not required for normal SDK use.

## Configuration and reliability

- `openai_rust_sdk::from_env()` requires `OPENAI_API_KEY`.
- `OPENAI_BASE_URL` is optional and must be an API origin such as
  `https://proxy.example.com`—do not append `/v1` or a trailing slash.
- The bearer API key is sent to the configured base URL. Only use endpoints you trust.
- The SDK does **not** currently apply automatic retries or expose a global request timeout.
  Applications should add workload-appropriate timeouts, backoff, and idempotency handling.
- OpenAI-compatible endpoints vary; a custom base URL does not guarantee support for every model,
  request field, stream event, or platform API.

See [Configuration and reliability](docs/configuration.md) for authentication, custom endpoints,
streaming behavior, errors, runtime/TLS details, and production considerations.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo doc --all-features --no-deps
python3 scripts/check_docs.py
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the development workflow and release process.

## Support and security

- Use [GitHub Issues](https://github.com/ThreatFlux/openai_rust_sdk/issues) for reproducible bugs
  and feature requests.
- Follow [SECURITY.md](SECURITY.md) to report vulnerabilities privately; do not open a public
  security issue.
- OpenAI account, billing, model-access, or service-status questions belong with
  [OpenAI Support](https://help.openai.com/).

## License

Licensed under the [MIT License](LICENSE).

## Acknowledgments

Created and maintained by Wyatt Roersma with development assistance from Claude Code. See the
repository history for the complete contributor record.
