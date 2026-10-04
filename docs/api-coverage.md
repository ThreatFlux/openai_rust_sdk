# OpenAI API coverage

> Coverage snapshot: **2026-10-03**, checked against fetched `main` and the modernization PR's
> source and the [official OpenAI API reference](https://developers.openai.com/api/reference/overview).

This crate is an unofficial, community-maintained SDK. OpenAI adds endpoints,
request fields, response fields, and streaming event types without waiting for a
crate release, so avoid treating module presence as a promise of complete API
parity. Check this page, the linked source, and the official reference before
choosing the SDK for a production integration.

Statuses are based on source-level endpoint and type coverage. They do not mean
that every operation is continuously exercised against the live OpenAI service.

## Status definitions

- **Supported** — the SDK implements the core operations currently listed for
  that resource group. New or uncommon optional fields can still lag the API.
- **Partial** — useful operations exist, but named endpoints, transports,
  event types, or current request options are missing.
- **Legacy** — code exists for an API that OpenAI has deprecated or placed in
  its legacy reference. Use it only to maintain or migrate an existing system.
- **Not supported** — there is no dedicated high-level client for the current
  API surface. A raw JSON escape hatch elsewhere does not change this status.

The matrix describes the reviewed modernization source, not necessarily the
latest crates.io release. See the [dated feature evidence](sdk-feature-plan.md)
for selected additions and compatibility boundaries. See [Configuration and operational behavior](configuration.md#release-source)
for the version distinction.

## Responses, tools, and realtime

| Surface | Status | Implemented | Gaps | SDK | OpenAI |
| --- | --- | --- | --- | --- | --- |
| Responses REST | **Supported** | Create, retrieve, delete, cancel, compact, count input tokens, and list input items. The request model covers conversation state, background mode, structured output, reasoning, safety identifiers, and raw input. Structured output, images, client tools, and tool choices serialize the current Responses wire shape. Separate current options add automatic compaction and cache retention; dedicated compaction models preserve cache-write usage and opaque output. Input counting projects the documented schema. | Optional fields can lag the live schema. Stored prompt templates cannot be expanded by input counting. Response headers are not exposed. The compatibility `list_responses` endpoint is not listed in the current official reference and does not establish an official stored-response list operation. | [source](../src/api/responses_v2.rs) · [models](../src/models/responses_v2.rs) · [example](../examples/responses_api.rs) | [Responses overview](https://developers.openai.com/api/reference/responses/overview) |
| Responses SSE streaming | **Partial** | `stream_response_v2` sends `stream: true` and yields typed `ResponseStreamEvent` values for common lifecycle, text, function-call, custom-tool, reasoning, refusal, and item events. The additive envelope stream preserves the full JSON, sequence number, and SSE transport metadata, including unknown events. Current text/item events and flat errors deserialize without legacy-only fields. | The compatibility stream still maps future events to payload-free `Unknown`. Not all tool events have dedicated typed variants. The SDK does not reconnect, resume, or expose stream response headers. Responses WebSocket mode is separate and unsupported. | [stream client](../src/api/responses_v2.rs) · [event enum](../src/models/responses_v2.rs) · [example](../examples/responses_api.rs) | [streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events) |
| Responses WebSocket mode | **Not supported** | — | No Responses WebSocket transport or client/server event loop. | — | [Responses overview](https://developers.openai.com/api/reference/responses/overview) |
| Responses multi-agent beta | **Not supported** | — | No dedicated beta multi-agent endpoints, streaming types, or WebSocket transport. | — | [API reference index](https://developers.openai.com/api/reference/overview) |
| Agents API | **Not supported** | — | No dedicated Agents sessions, environments, artifacts, plugins, or execution transport. | — | [Agents overview](https://developers.openai.com/api/docs/guides/agents-api) |
| Conversations and items | **Supported** | Create, retrieve, update, and delete conversations; create, retrieve, list, and delete items. | New optional fields can require a crate update. | [source](../src/api/conversations.rs) · [models](../src/models/conversations.rs) | [Responses resources](https://developers.openai.com/api/reference/responses/overview) |
| Responses tools | **Partial** | Typed request support exists for functions, web search, file search, MCP, image generation, Code Interpreter, computer use, tool search, local shell, shell, apply patch, custom tools, and namespaces. | Tool schemas evolve quickly. Not every current tool option, connector flow, approval flow, or emitted tool event has a typed representation. Application-defined function execution remains the application's responsibility. | [tool models](../src/models/tools/mod.rs) · [tool example](../examples/responses_mcp_tool.rs) | [tools guides](https://developers.openai.com/api/docs/guides/tools) |
| Webhooks | **Partial** | Verify signatures over exact raw bodies, authenticate before parsing, and preserve current or unknown event payloads in a typed common envelope. | Resource-specific event data remain JSON objects. Delivery deduplication, webhook endpoint management, and application event processing remain caller responsibilities. | [source](../src/webhooks/mod.rs) | [webhook events](https://developers.openai.com/api/reference/resources/webhooks) |
| Realtime API | **Partial** | REST calls exist for GA client secrets and realtime transcription sessions. There are event/audio types and a WebRTC prototype. | No WebSocket or SIP client, Calls API, translation sessions, or complete GA event coverage. `create_session` still calls the legacy `/v1/realtime/sessions` endpoint, and `connect_webrtc` does not exchange SDP with OpenAI; it only marks local state connected. Do not treat it as a production-ready end-to-end Realtime transport. | [REST client](../src/api/realtime_audio_impl/client.rs) · [WebRTC prototype](../src/api/realtime_audio_impl/webrtc/connection.rs) · [prototype example](../examples/realtime_audio_demo.rs) | [Realtime guide](https://developers.openai.com/api/docs/guides/realtime) · [client secrets](https://developers.openai.com/api/reference/resources/realtime/subresources/client_secrets/methods/create) |

## Platform APIs

| Surface | Status | Implemented | Gaps | SDK | OpenAI |
| --- | --- | --- | --- | --- | --- |
| Audio | **Partial** | Speech generation (including byte streaming), file transcription, and translation. | Voice consent and custom voice endpoints are not implemented. | [source](../src/api/audio_impl/mod.rs) · [example](../examples/audio_demo.rs) | [Audio API](https://developers.openai.com/api/reference/resources/audio/subresources/speech/methods/create) |
| Videos | **Partial** | Create, retrieve, list, delete, download, edit, extend, and remix videos. | Character create/retrieve endpoints are not implemented. | [source](../src/api/videos.rs) | [Videos API](https://developers.openai.com/api/reference/resources/videos/methods/create) |
| Images | **Partial** | Generate, edit, and create variations, plus file download helpers. | Image generation/edit streaming event APIs are not implemented. | [source](../src/api/images/mod.rs) · [example](../examples/images_demo.rs) | [Images API](https://developers.openai.com/api/reference/resources/images/methods/generate) |
| Embeddings | **Supported** | Create embeddings, with convenience helpers for single/multiple inputs and vector operations. | — | [source](../src/api/embeddings.rs) · [example](../examples/embeddings_demo.rs) | [Embeddings API](https://developers.openai.com/api/reference/resources/embeddings/methods/create) |
| Evals | **Supported** | Eval CRUD/list; run create/retrieve/delete/list/cancel; output-item retrieve/list. | New data-source and grader variants can require model updates. | [source](../src/api/evals.rs) · [models](../src/models/evals.rs) | [Evals API](https://developers.openai.com/api/reference/resources/evals/methods/create) |
| Fine-tuning | **Partial** | Job create/retrieve/list/cancel/pause/resume, events, checkpoints, and polling helpers. | Alpha grader run/validation and checkpoint permission endpoints are not implemented. | [source](../src/api/fine_tuning.rs) · [example](../examples/fine_tuning_demo.rs) | [Fine-tuning API](https://developers.openai.com/api/reference/resources/fine_tuning/subresources/jobs/methods/create) |
| Batch | **Supported** | Create, retrieve, list, cancel, poll, upload input files, and download output/error files. | The optional [YARA-X case study](examples/batch-yara-x.md) adds local helpers; they are not OpenAI endpoints. | [source](../src/api/batch/mod.rs) · [example](../examples/batch_list_jobs.rs) | [Batch API](https://developers.openai.com/api/reference/resources/batches/methods/create) |
| Files | **Supported** | Upload, list, retrieve metadata/content, download, and delete files. | — | [source](../src/api/files.rs) · [example](../examples/files_demo.rs) | [Files API](https://developers.openai.com/api/reference/resources/files/methods/list) |
| Uploads | **Supported** | Create multipart upload, add binary parts, complete, and cancel. | — | [source](../src/api/uploads.rs) · [models](../src/models/uploads.rs) | [Uploads API](https://developers.openai.com/api/reference/resources/uploads/methods/create) |
| Models | **Supported** | List, retrieve, and delete models, with local filtering helpers. | Local capability classification is SDK-maintained metadata and can lag new models. | [source](../src/api/models/client.rs) · [example](../examples/models_demo.rs) | [Models API](https://developers.openai.com/api/reference/resources/models/methods/retrieve) |
| Moderations | **Supported** | Create moderation, plus local convenience methods for thresholds and category inspection. | — | [source](../src/api/moderations/mod.rs) · [example](../examples/moderations_demo.rs) | [Moderations API](https://developers.openai.com/api/reference/resources/moderations/methods/create) |
| Content provenance checks | **Not supported** | — | No request or response types and no endpoint client. | — | [Content provenance API](https://developers.openai.com/api/reference/resources/content_provenance_checks/methods/create) |
| Vector stores | **Supported** | Store CRUD/list/search; file create/retrieve/update/list/delete/content; file-batch create/retrieve/list/cancel; polling helpers. | — | [source](../src/api/vector_stores.rs) · [example](../examples/vector_stores_demo.rs) | [Vector Stores API](https://developers.openai.com/api/reference/resources/vector_stores/methods/create) |
| Containers | **Partial** | Container create/retrieve/list/delete; file create/list/retrieve/delete/content; local download helpers. | Additive `retrieve_file` and `list_files_with_params` return the current official metadata and cursor-page shapes. Legacy upload/list DTOs and container creation models still lag the current wire schema. `update_container`, code execution, and keep-alive helpers are not evidence of additional official endpoint coverage. | [source](../src/api/containers.rs) · [example](../examples/code_interpreter_demo.rs) | [Containers API](https://developers.openai.com/api/reference/resources/containers/methods/create) |
| Skills | **Supported** | Skill CRUD/list/content and version create/retrieve/list/delete/content. | — | [source](../src/api/skills.rs) · [models](../src/models/skills.rs) | [Skills API](https://developers.openai.com/api/reference/resources/skills/methods/create) |
| ChatKit | **Not supported** | — | No ChatKit sessions or thread client. The legacy Assistants `ThreadsApi` is unrelated. | — | [ChatKit API](https://developers.openai.com/api/reference/resources/beta/subresources/chatkit/subresources/sessions/methods/create) |

## Administration and older APIs

| Surface | Status | Implemented | Gaps | SDK | OpenAI |
| --- | --- | --- | --- | --- | --- |
| Administration | **Supported** | Organization admin-key lifecycle, audit logs and filters, invites, users and RBAC roles, projects, project users/groups, service accounts and their keys, project API keys/rate limits, certificates, data-retention controls, model/hosted-tool permissions, spend limits/alerts, costs, and every documented organization usage category. | Resource payloads that are intentionally extensible on the fast-moving Administration API are exposed as `serde_json::Value`; callers should still use the official schemas for newly introduced fields. Admin endpoints require an admin credential; the SDK does not distinguish credential types. | [source](../src/api/admin.rs) · [models](../src/models/admin.rs) | [Administration overview](https://developers.openai.com/api/reference/administration/overview) |
| Chat Completions | **Partial** | Create a chat completion and consume its SSE stream through the compatibility `ResponsesApi`/`StreamingApi`. | Stored completion retrieve/update/delete/list operations are not implemented. New integrations should normally start with Responses. | [source](../src/api/responses.rs) · [streaming source](../src/api/streaming/client.rs) · [example](../examples/chat_completion.rs) | [Chat Completions overview](https://developers.openai.com/api/reference/chat-completions/overview) |
| Assistants, Threads, Messages, and Runs | **Legacy** | Assistant CRUD/list, thread/message operations, run/step operations, tool-output submission, and legacy streaming are present. | OpenAI deprecated the Assistants API with a published shutdown date of **August 26, 2026**. These modules remain source-compatibility surfaces; their presence does not establish live availability after that date. Do not start a new integration. Migrate existing code to Responses and Conversations. | [assistants](../src/api/assistants.rs) · [threads](../src/api/threads/mod.rs) · [runs](../src/api/runs.rs) · [legacy example](../examples/assistants_demo.rs) | [deprecation notice and migration guidance](https://developers.openai.com/api/docs/guides/migrate-to-responses) |
| Legacy Completions | **Not supported** | — | No `/v1/completions` client. | — | [Legacy Completions API](https://developers.openai.com/api/reference/resources/completions/methods/create) |

## Reporting a mismatch

When the live API and this matrix disagree, the official OpenAI reference is
authoritative. Please open an issue with:

1. The official reference URL and the date you checked it.
2. The endpoint, request field, response field, or event type that differs.
3. A minimal redacted request/response or a compile-only reproduction.
4. The exact crate version or Git commit.

Never include API keys, bearer tokens, uploaded customer data, or unredacted
production responses.
