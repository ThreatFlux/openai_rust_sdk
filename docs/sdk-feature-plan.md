# SDK modernization and feature evidence

Reviewed on **2026-10-03** against fetched `main` commit
`75413ed` and the linked official OpenAI documentation. Implementation is in the
modernization PR; package publication and live API validation are separate.

The refresh upgrades stable Rust, stable direct crates, and pinned GitHub Actions
using current upstream release evidence. This page records the selected API work
and the intentionally deferred provider surfaces.

## Implemented slices

| Observed gap | Verified schema or behavior | Implementation and compatibility | Offline verification |
| --- | --- | --- | --- |
| Current text/item SSE events omit the legacy required `response_id`; flat error events failed deserialization. | [Responses event reference](https://developers.openai.com/api/reference/python/resources/responses), [streaming guide](https://developers.openai.com/api/docs/guides/streaming-responses). | Legacy fields accept omission; existing enum variants and Rust constructor layouts remain available. Current flat errors normalize into the legacy typed error. | Current wire fixtures and mocked SSE requests; malformed recognized payloads fail. |
| Unknown SSE events and fields such as sequence numbers, log probabilities, and transport IDs were discarded. | Same official event reference. | Additive [ResponseStreamEnvelope](../src/models/responses_v2/stream_envelope.rs) and lossless stream methods retain full JSON and SSE metadata. The old stream still returns the existing enum. | Future-event round trip, known-event extensions, CRLF framing, Unicode, retry interval, keepalives, and terminal sentinel. |
| Several Responses reads ignored HTTP status; streaming used separate error handling. | [Responses API](https://developers.openai.com/api/reference/python/resources/responses). | Reuse shared GET and streaming transport. Preserve structured API status and non-JSON upstream errors. Shared query encoding prevents query values becoming extra parameters. | Mocked permission/upstream failures and reserved-character queries. |
| Input counting sent generation-only fields. | [Input token count](https://developers.openai.com/api/reference/python/resources/responses/subresources/input_tokens/methods/count). | Allowlist projection keeps prompt-affecting supported fields. Unsupported prompt-template expansion and non-text instructions fail locally. Structured-output and tool serialization use the same corrected generation projection. | Exact request-body assertions and unsupported-input regressions. |
| Compaction was exposed only as a generic response and omitted a current cache option. | [Compaction](https://developers.openai.com/api/reference/python/resources/responses/methods/compact). | Additive dedicated request/response types preserve opaque replay items and cache-write usage. The original convenience method retains its signature. | Compaction round trip and exact HTTP request body. |
| Responses serialized Chat Completions shapes for structured output, images, function/custom tools, and simple tool choice. | [Response creation](https://developers.openai.com/api/reference/python/resources/responses/methods/create). | Keep public builders; generate current `text.format`, flat image/tool fields, grammar format, and string choice tags. Allowed tool names resolve only declared function/custom kinds; unsupported separate regex flags fail locally. Opaque tool/compaction items no longer gain an invented empty content array during replay. | Exact expected JSON for generation and counting, tool restrictions, malformed inputs, and opaque compaction round trips. |
| Current create options were missing. | [Response creation](https://developers.openai.com/api/reference/python/resources/responses/methods/create). | Separate options add automatic context compaction and prompt-cache retention without changing existing request struct literals. Model support remains server-validated. | Options serialization and compile-tested [example](../examples/responses_current.rs). |
| Container-file retrieve metadata was missing; legacy file types required fields absent from current payloads. | [Retrieve container file](https://developers.openai.com/api/reference/python/resources/containers/subresources/files/methods/retrieve), [list container files](https://developers.openai.com/api/reference/python/resources/containers/subresources/files/methods/list). | Additive metadata DTO, retrieve method, and cursor-page list method. Legacy helper types remain available. | Official-shape fixture, retrieve/list mocks, path validation, supported list controls, and API errors. |
| Webhook signatures and events were unsupported. | [Webhooks guide](https://developers.openai.com/api/docs/guides/webhooks). | Independent webhook slice adds authenticated parsing of exact raw request bodies and lossless event metadata. | Independent signature vectors, timestamp checks, malformed headers, tampering, and unknown events. |

## Compatibility and operational boundaries

The selected Responses, metadata, and webhook API additions are additive. Existing
exhaustive matches over the Responses
stream enum still compile; its `Unknown` variant continues to discard payloads
unless callers select the new envelope stream. Current request options are
supplied separately rather than adding fields to existing public struct layouts.

The stable `webrtc` and `rtc` dependency upgrades from 0.20 to 0.21 affect
Realtime methods exposing upstream types. Consumers constructing their own
`PeerConnection`, `DataChannel`, `TrackLocalStaticSample`, `RTCDataChannelMessage`,
or `TrackRemote` must upgrade matching direct dependencies too. This dependency
type migration is a breaking compatibility change even though the new SDK
request and event APIs are additive; release classification must account for it.
The [2.0 migration guide](migration-2.0.md) lists affected methods and dependency
adaptations. The manifest remains at the last published version until the
maintainer's release PR updates it.
The existing Realtime transport remains a prototype.

The compatibility `list_responses` method still queries `GET /v1/responses`.
The current official resource reference does not list that operation; its presence
is not evidence of official stored-response listing support.

The legacy container helper models still describe their historical convenience
shape. Use `retrieve_file` and `list_files_with_params` for current official
metadata. This update does not claim the legacy container create, code-execution,
update, or keep-alive helper endpoints are current OpenAI endpoints.

Input-count projection does not expand stored prompt templates. It does not claim
that generation options absent from the count schema are counted. Compaction
requires text instructions and explicit context or a previous response; replay
all returned output items rather than extracting only visible text.

No live API calls, webhook endpoint registrations, billable requests, admin
mutations, merges, tags, or crate publication are performed by local validation.
Tests deliberately unset `OPENAI_API_KEY`.

## Deferred API work

Dedicated Responses WebSocket and beta multi-agent transports, Agents API,
production Realtime WebSocket/SIP/Calls support, image streaming, custom audio
voices and consents, video characters, content provenance, ChatKit, fine-tuning
alpha graders/checkpoint permissions, and stored Chat Completions operations
remain separate implementation slices. Each needs its own verified typed models,
transport semantics, examples, and tests before changing its coverage status.

The Assistants modules remain legacy source-compatibility surfaces. Their presence
does not establish live endpoint availability after OpenAI's published retirement
date. New integrations should use Responses and Conversations.
