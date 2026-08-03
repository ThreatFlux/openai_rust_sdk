# Configuration and operational behavior

This document describes behavior verified in this repository's `main` source
on **2026-08-02**. It is intentionally explicit about limitations that matter
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
retained. There is no automatic reconnect, event replay, or cursor resumption.
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

As of **2026-08-03**, crates.io publishes `openai_rust_sdk` **1.6.0** with
`rust-version = 1.97.1`. The release tag, packaged crate, and matching docs.rs page
describe the same source.

The default branch can advance after a release. Let `cargo add openai_rust_sdk` select
the published version, inspect the resolved version in `Cargo.lock`, and use the
matching docs.rs page. If you intentionally depend on Git, pin a reviewed commit SHA instead of
a moving branch.

The repository's `rust-toolchain.toml` and `Cargo.toml` are authoritative for a
source build. The selected crate release's manifest is authoritative for a
registry dependency.
