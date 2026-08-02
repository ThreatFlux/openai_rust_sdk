# OpenAI Rust SDK capabilities

This page is a high-level map of the crate. For the dated, evidence-based status of each API area,
including known gaps and legacy endpoints, use the canonical
[API coverage matrix](docs/api-coverage.md).

## Core capabilities

| Capability | What the crate provides |
| --- | --- |
| Responses | Typed create, retrieve, list, cancel, compact, token-counting, input-item, and streaming APIs |
| Tools | Function calling, web and file search, MCP, image generation, computer use, shell, apply-patch, and custom tools |
| Conversations | Typed conversation and item operations for Responses-based applications |
| Platform APIs | Clients for Batch, files, uploads, vector stores, media, evals, fine-tuning, moderation, and administration |
| Realtime | Useful REST session operations and experimental connection helpers, with important gaps documented in the coverage matrix |
| Legacy migration | Assistants, Threads, and Runs remain available while users migrate to Responses |

“Typed” does not imply that every current OpenAI endpoint or event is implemented. OpenAI evolves
the API independently of this community project. Check
[`docs/api-coverage.md`](docs/api-coverage.md) before selecting the crate for a specific workflow.

## Cargo features

The OpenAI API client is available with no optional features:

```bash
cargo add openai_rust_sdk
```

The security-oriented Batch API example can optionally compile generated YARA rules locally. This
adds a substantial compilation dependency, so the tooling is opt-in:

```bash
cargo add openai_rust_sdk --features yara
```

| Feature | Effect |
| --- | --- |
| `default` | Core OpenAI API client |
| `yara` | YARA-X validation and CLI support for the optional Batch API example |
| `testing` | Reserved compatibility feature; currently adds no dependencies |
| `full` | All optional capabilities; currently enables `testing` and `yara` |

See [the YARA-X Batch example](docs/examples/batch-yara-x.md) for exact commands, output semantics,
and security guidance.

## Production considerations

The shared HTTP client currently has no automatic retry policy or global request timeout. Custom
base URLs receive the configured bearer credential and must be trusted. Streaming consumers must
handle both transport errors and typed API error events.

Read [Configuration and reliability](docs/configuration.md) for authentication, base URL rules,
TLS/runtime details, error handling, and a production-readiness checklist.

## Explore the API

- Start with the compile-tested [`examples/quickstart.rs`](examples/quickstart.rs).
- Browse task-oriented programs in [`examples/`](examples/).
- Use the generated [docs.rs API reference](https://docs.rs/openai_rust_sdk).
- Review [the changelog](CHANGELOG.md) before upgrading.

The Assistants API is deprecated and scheduled to shut down on August 26, 2026. New integrations
should use Responses; existing integrations should follow OpenAI's
[migration guide](https://developers.openai.com/api/docs/guides/migrate-to-responses).
