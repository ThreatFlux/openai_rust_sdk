# Migration to the next major SDK release

Reviewed on **2026-10-03**. This source update requires a **2.0** release because
the stable `webrtc` and `rtc` upgrades change types exposed by the SDK's public
Realtime API. The manifest remains at the last release, **1.7.0**, until the
maintainer's version PR updates it. Development uses Rust **1.99.0**; the consumer
MSRV remains **1.97.1** and is checked separately with all features.

## Align direct WebRTC dependencies

If your application passes its own WebRTC objects to `RealtimeSession`, update
its direct dependencies together:

```toml
[dependencies]
webrtc = "0.21.0"
rtc = "0.21.0"
```

An object from `webrtc` 0.20 cannot satisfy a trait or parameter from 0.21, even
when its name is unchanged. This affects the peer connection passed to
`RealtimeSession::new` and returned by `peer_connection`, `set_data_channel`,
`set_audio_track`, `handle_data_channel_message`, `handle_incoming_track`, and
`RealtimeAudioConfig::ice_servers`. Remove conflicting 0.20 dependencies or
update the application layer that owns those objects. Inspect `cargo tree -d`
and compile the application after upgrading; do not cast between versions.

`rtc::media::Sample` now takes a clock instant instead of using `Default`:

```rust
use rtc::media::Sample;
use std::time::{Duration, Instant};

let sample = Sample {
    data: vec![0_u8; 960].into(),
    duration: Duration::from_millis(20),
    ..Sample::new(Instant::now())
};
```

`TrackLocalStaticSample::new` likewise takes `Instant::now()` before the track
argument. These adaptations are applied in the SDK. The existing Realtime
transport remains a prototype; this migration does not establish an end-to-end
OpenAI WebRTC connection.

## Adopt the current Responses methods

Existing request struct literals and matches over `ResponseStreamEvent` remain
available. For full event JSON and SSE metadata, use
`client.responses_v2().stream_response_envelopes(&request)` instead of the
compatibility enum stream. Handle each yielded `Result` and retain `payload`
when an event is unfamiliar; the old `Unknown` variant still loses its payload.

`ResponseCreateOptions` is a separate argument to
`create_response_with_options` or `stream_response_envelopes_with_options`.
It adds automatic context compaction and prompt-cache retention without adding
required fields to `CreateResponseRequest`. Model support is validated by the
server. Input counting projects only supported prompt fields and rejects
unsupported prompt-template expansion or non-text instructions locally.

Use `compact_context` with `CompactResponseRequest` for typed compaction.
Replay all returned output items, including opaque compaction items. The
existing `compact_response` convenience signature remains available. The
[current Responses example](../examples/responses_current.rs) demonstrates the
new APIs; running it makes live API calls.

Responses requests now emit the current `text.format`, image, tool and
tool-choice wire shapes. Applications using a custom compatibility server
should verify it accepts these corrected shapes. Separate regex flags are
rejected because the current custom-tool schema cannot represent them.

## Read current container-file metadata

Use `ContainersApi::retrieve_file` and `list_files_with_params` with the new
metadata and page types. The legacy container-file DTO and helpers retain
their historical shapes; their presence is not evidence that all legacy
container helper endpoints are supported by OpenAI.

## Authenticate webhooks before parsing

Use `WebhookVerifier` with the webhook signing secret, exact raw body bytes,
and the three webhook headers. `unwrap` authenticates before parsing;
deserializing `WebhookEvent` directly does not authenticate a delivery.
The [verification example](../examples/webhook_verification.rs) reads the
signing secret and headers from environment variables and the body from stdin.
It does not call the OpenAI API. Deduplicate deliveries and process resource
events in the application; resource-specific payloads remain JSON objects.

For implemented and deferred surfaces, see the
[feature evidence](sdk-feature-plan.md) and [coverage matrix](api-coverage.md).
The optional YARA-X exceptions remain conditional and are described in
[configuration](configuration.md#optional-yara-x-dependency-security-review).
