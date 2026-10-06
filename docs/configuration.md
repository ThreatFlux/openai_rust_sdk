# Configuration and operational behavior

This document describes behavior verified in this repository's source
on **2026-10-03**. It is intentionally explicit about limitations that matter
in production. For endpoint availability, see [OpenAI API coverage](api-coverage.md).

## Environment-based setup

The crate-level `from_env()` function reads exactly two variables:

| Variable | Required | Behavior |
| --- | --- | --- |
| `OPENAI_API_KEY` | Yes | Bearer credential; missing or blank is an error. |
| `OPENAI_BASE_URL` | No | Defaults to `https://api.openai.com`. |

When `OPENAI_BASE_URL` is present, its value is trimmed and passed to the
client. A blank or non-Unicode value returns an invalid-request error.

```rust
use openai_rust_sdk::from_env;

fn main() -> openai_rust_sdk::Result<()> {
    let client = from_env()?;
    let _ = client;
    Ok(())
}
```

The SDK does **not** load `.env` files. Load one in your application before
calling `from_env()` if that is part of your configuration strategy.

`OPENAI_MODEL`, `OPENAI_ORGANIZATION`, and `OPENAI_PROJECT` are not read by the
SDK. Runnable examples that make model requests require `OPENAI_MODEL` rather
than silently choosing a model. That is application behavior rather than
client configuration; production applications should make their own explicit
model-selection policy.

`from_env_with_base_url(url)` reads `OPENAI_API_KEY` and uses the URL argument;
it does not consult `OPENAI_BASE_URL`. `OpenAIClient::from_env()` does not exist:
import and call the crate-level function as shown above.

## Explicit setup

```rust
use openai_rust_sdk::OpenAIClient;

fn main() -> openai_rust_sdk::Result<()> {
    let client = OpenAIClient::new("replace-with-a-secret-from-your-key-store")?;

    let proxy_client = OpenAIClient::with_base_url(
        "replace-with-a-secret-from-your-key-store",
        "https://openai-proxy.example.com",
    )?;
    let _ = (client, proxy_client);
    Ok(())
}
```

`OpenAIClient` configures its Responses, compatibility streaming, and function
calling clients. Other API types, such as `FilesApi` or `AdminApi`, are
standalone clients and must be constructed with their own API key and custom
base URL when needed.

## Custom base URLs

The base URL is concatenated directly with endpoint paths such as
`/v1/responses`; it is not normalized or joined as a structured URL.

Use an origin or proxy root:

```text
https://openai-proxy.example.com
```

Do **not** include `/v1`:

```text
https://openai-proxy.example.com/v1   # produces .../v1/v1/responses
```

Avoid a trailing slash:

```text
https://openai-proxy.example.com/     # produces ...//v1/responses
```

The SDK does not enforce HTTPS or validate a custom URL during construction;
an invalid value can fail only when a request is built or sent.

### Credential-forwarding warning

Every high-level request sends the supplied credential to the configured host
as `Authorization: Bearer ...`. Setting `OPENAI_BASE_URL` or calling
`with_base_url` therefore gives that host access to the credential. Use only a
proxy or OpenAI-compatible endpoint you trust, and prefer a narrowly scoped
credential intended for that service. Never point a client holding a
production OpenAI key at an untrusted diagnostic endpoint.

## Authentication and headers

Normal JSON requests include bearer authorization and
`Content-Type: application/json`. Multipart requests let `reqwest` generate the
multipart content type. Legacy Assistants requests also add
`OpenAI-Beta: assistants=v2`.

OpenAI's Administration endpoints require an admin API key. `AdminApi` accepts
an arbitrary non-empty bearer credential but does not verify that it is an
admin key before sending a request.

The Administration client covers organization and project administration,
including admin-key rotation, users/invites, projects, groups and roles,
certificates, retention, spend controls, permissions, service-account keys, and
all organization usage categories. Fast-moving resource payloads are returned
as `serde_json::Value` where OpenAI does not guarantee a stable schema:

```rust,no_run
use openai_rust_sdk::{AdminApi, CreateAdminApiKeyRequest};

# async fn example() -> openai_rust_sdk::Result<()> {
let admin = AdminApi::new(std::env::var("OPENAI_ADMIN_KEY").unwrap())?;
let key = admin.create_admin_api_key(&CreateAdminApiKeyRequest {
    name: "automation".into(),
    expires_in_seconds: Some(30 * 24 * 60 * 60),
}).await?;
println!("created {}", key.id);
let groups = admin.list_groups(None).await?;
println!("{} groups", groups.data.len());
# Ok(())
# }
```

The high-level clients currently have no supported per-request header API for:

- `OpenAI-Organization`
- `OpenAI-Project`
- `X-Client-Request-Id`
- idempotency or application-specific tracing headers

The public low-level `HttpClient::client()` accessor can be used to build a raw
`reqwest` request, but then the application owns URL construction, headers,
status handling, and deserialization. OpenAI documents bearer authentication,
organization/project routing, and request IDs in its
[API overview](https://developers.openai.com/api/reference/overview).

## Retries and rate limits

The SDK adds **no automatic retry or backoff policy**. In particular, it does
not retry `429` or `5xx` responses, honor `Retry-After`, add jitter, or protect
non-idempotent operations from duplicate submission. Build retry policy at the
application boundary and decide which operations are safe to repeat.

OpenAI's [rate-limit guidance](https://developers.openai.com/api/docs/guides/rate-limits)
should drive that policy. Because the high-level SDK discards response headers,
it does not expose the `x-ratelimit-*` headers needed for adaptive throttling.
Use a lower-level request path if those headers are operationally required.

`BatchApi`, `FineTuningApi`, and `VectorStoresApi` include polling helpers.
Polling is not request retry: a failed poll still returns an error, and callers
must choose suitable intervals and overall deadlines.

## Timeouts

The shared REST/SSE `HttpClient` uses `reqwest::Client::new()` without setting a
total, connect, or read timeout, and the SDK exposes no high-level timeout
builder. Long-running or stalled calls therefore need an application deadline,
for example with `tokio::time::timeout` or cancellation in a `select!` loop.

`FunctionsApi` is an exception: its independently constructed `reqwest` client
sets a two-minute total timeout. Polling-helper maximum wait values and
container code-execution timeout fields govern those workflows; they do not
configure the underlying shared HTTP client's network timeout.

For a streaming response, apply an idle or overall deadline while consuming
the stream as well as while creating it. Dropping a local stream stops local
consumption; it is not a substitute for calling the Responses cancel endpoint
for a background response.

## Request IDs and diagnostics

OpenAI recommends logging `x-request-id` for production troubleshooting. The
high-level clients deserialize the body and discard response headers, so they
do not expose `x-request-id`, `openai-processing-ms`, or rate-limit headers.
They also do not automatically generate `X-Client-Request-Id`.

If request correlation is a requirement, use the low-level reqwest client or a
trusted reverse proxy that injects and logs a client request ID. Do not put
secrets or personal data in request IDs. See OpenAI's
[request debugging guidance](https://developers.openai.com/api/reference/overview#debugging-requests).

## Errors

Public operations return `openai_rust_sdk::Result<T>`, whose error type is
`OpenAIError`. Important variants include:

- `Request` for a retained `reqwest::Error`.
- `Json` and `ParseError` for serialization or response-decoding failures.
- `Api { status_code, message }` when a standard OpenAI error envelope is
  decoded.
- `ApiError { status, message }` when a code path preserves an HTTP status and
  raw error text.
- `Authentication` and `InvalidRequest` for local validation/configuration
  failures.
- `Streaming` for SSE transport/parser failures.
- `Timeout` for SDK workflow deadlines such as polling helpers. A reqwest
  network timeout can instead appear as `Request` or a string-based request
  error, depending on the API module.

Error mapping is not yet uniform across modules. Match both `Api` and
`ApiError` when status-specific behavior matters, retain the full error for
logs, and redact credentials and sensitive response content. The SDK does not
currently expose structured API error `type`, `param`, and `code` fields after
conversion. Consult OpenAI's [error-code guide](https://developers.openai.com/api/docs/guides/error-codes)
for the service-side meaning of a failure.

## Streaming behavior

Two similarly named paths target different APIs:

- `OpenAIClient::stream_response_v2` uses `/v1/responses` and returns typed
  Responses SSE events. Prefer this for new integrations.
- `OpenAIClient::create_response_stream` and `StreamingApi` use
  `/v1/chat/completions` compatibility types.

Responses streaming recognizes common event types and maps an unrecognized
type to `ResponseStreamEvent::Unknown`; the unknown event payload is not
retained by that compatibility stream. Use
`ResponsesApiV2::stream_response_envelopes` to retain full JSON, sequence numbers,
and SSE metadata for current and future events. There is no automatic reconnect,
event replay, or cursor resumption.
Every yielded item is a `Result`, so handle errors inside the consumption loop
rather than only when opening the stream.

## Runtime and TLS

- The SDK is asynchronous and built around Tokio. A Tokio runtime must be
  active while calling async APIs; `#[tokio::main]` is sufficient for a binary.
- REST and SSE use `reqwest` with default features disabled and the `rustls`
  feature enabled. They do not depend on a system OpenSSL installation.
- Realtime/WebRTC dependencies are currently unconditional crate dependencies,
  even though the Realtime implementation is partial.
- WebAssembly support is not documented or tested; assume a native target
  unless your own build and runtime tests prove otherwise.

## Release source

The crate declares `rust-version = 1.97.1`. Version **2.0.0** introduced a
breaking WebRTC dependency-type migration; see the
[migration guide](migration-2.0.md).
Development and CI use the pinned **1.99.0** toolchain; the separate MSRV check
uses **1.97.1** with all features.
After publication, the release tag, packaged crate, and matching docs.rs page
should describe the same source.

The default branch can advance after a release. Let `cargo add openai_rust_sdk` select
the published version, inspect the resolved version in `Cargo.lock`, and use the
matching docs.rs page. If you intentionally depend on Git, pin a reviewed commit SHA instead of
a moving branch.

The repository's `rust-toolchain.toml` and `Cargo.toml` are authoritative for a
source build. The selected crate release's manifest is authoritative for a
registry dependency.

## Optional YARA-X dependency security review

Reviewed on **2026-10-03** against the locked all-feature graph: YARA-X **1.21.0**,
Wasmtime **45.0.3**, RSA **0.9.10**, and Bincode **2.0.1**. The latest stable
[YARA-X manifest](https://github.com/VirusTotal/yara-x/blob/v1.21.0/Cargo.toml)
requires Wasmtime 45; no compatible stable upgrade reaches the patched Wasmtime
48/49 releases. The following exceptions apply to this repository's verified
usage. They do not patch upstream dependencies.

| Advisory | Reviewed applicability |
| --- | --- |
| [RUSTSEC-2023-0071](https://rustsec.org/advisories/RUSTSEC-2023-0071) | The Marvin attack requires private RSA operations. YARA-X uses public keys for signature verification, with no private key, signing, or decryption operation. |
| [RUSTSEC-2025-0141](https://rustsec.org/advisories/RUSTSEC-2025-0141) | Bincode is unmaintained. Stable YARA-X still depends on it; this is an accepted maintenance risk, not a claim that Bincode remains maintained. The SDK does not deserialize compiled rules. |
| [RUSTSEC-2026-0222](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-hgjw-h833-99q9) | The affected APIs require mixing objects from different Wasmtime engines. YARA-X creates compiler modules and scanner stores through the same global engine, following the upstream single-engine workaround. |
| [RUSTSEC-2026-0269](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-vqjp-4c8c-hfgg) | The filesystem escape affects Wasmtime-WASI/cap-std. Neither dependency is present; the validator does not provide WASI filesystem imports. |
| [RUSTSEC-2026-0316](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-jqpg-j7w6-42pr) | The affected API is `wasmtime::component::Val`. The component model is disabled and that API is unavailable. |
| [RUSTSEC-2026-0327](https://github.com/bytecodealliance/wasmtime/security/advisories/GHSA-32h6-97mm-8q3c) | The affected component async callbacks are unavailable. Disabling `component-model-async` is the advisory's documented workaround. |

The source evidence is YARA-X's
[public-key verification](https://github.com/VirusTotal/yara-x/blob/v1.21.0/lib/src/modules/utils/crypto.rs),
[global engine](https://github.com/VirusTotal/yara-x/blob/v1.21.0/lib/src/wasm/mod.rs),
[compiler module creation](https://github.com/VirusTotal/yara-x/blob/v1.21.0/lib/src/compiler/mod.rs),
and [scanner store creation](https://github.com/VirusTotal/yara-x/blob/v1.21.0/lib/src/scanner/context.rs).
The resolved Wasmtime feature union is limited to core compilation/runtime and
support features; it contains neither component-model feature.

`make audit`, `make deny`, and both hosted security jobs first run
`python3 scripts/check_yara_security.py`. The guard rejects changed reviewed
versions, non-registry replacements, new Wasmtime features, WASI/cap-std
packages, additional callers of the reviewed dependencies, mismatched exception
policies, and direct SDK exposure of the
excluded Wasmtime or compiled-rule APIs. New advisory IDs still fail the normal
security checks. Re-review these exceptions when a fixed compatible YARA-X
release becomes available; remove them when their dependencies are fixed or
removed.

Cargo unifies features across dependencies. A downstream application that adds
Wasmtime component APIs, WASI, or new YARA-X entry points must perform its own
applicability review; these repository exceptions do not establish safety for
that application's feature graph. The YARA-X feature remains optional and is
unnecessary for ordinary OpenAI SDK use.
