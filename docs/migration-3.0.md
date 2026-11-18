# Migrating to OpenAI SDK 3.0

The next major release replaces unmaintained chrono with Jiff 1.0.1. Public
Realtime session, statistics, audio-buffer and voice-detection timestamps, plus
`RealtimeSession::started_at`, use the SDK's re-exported `Timestamp` type.
Use `openai_rust_sdk::Timestamp::now()` or `Timestamp::from_second(seconds)`
instead of chrono's UTC constructors. The latter returns a Result with a range
check. SDK example formatting now checks unsigned Unix-second conversion rather
than wrapping out-of-range unsigned values into negative dates.

JSON timestamps remain UTC RFC3339 strings. Numeric Unix timestamps in OpenAI
request/response models remain numeric. Jiff preserves ordinary offset instants
and nanoseconds, serializes optional timestamps as null, and rejects missing
timezones and invalid dates. Its string formatter uses canonical fractional
precision, which can differ in trailing zeros from chrono's serializer.

Unix timestamps cannot distinguish a leap-second instant. Jiff accepts an
RFC3339 second 60 and normalizes it to second 59, retaining fractional digits.
Do not depend on preserving the lexical second 60 across a round trip. Public
Realtime response regression fixtures make this behavior explicit.

Connection durations still return signed whole seconds. Jiff's signed duration
method replaces chrono's `num_seconds` conversion; the public return type remains
`Option<i64>`. The realtime transport and WebRTC test fixtures use the same
maintained timestamp type as production code.

This source change requires a 3.0.0 release proposal. No chrono advisory
exception is added. Exact-head hosted tests/security, the read-only release
preview, the actual registry/native/container/docs publications and downstream
Countermeasure consumer verification are separate release requirements.

The optional YARA capability retains every default runtime module and optimizer
feature. It uses YARA-X 1.23.0's shipped protobuf Rust sources, avoiding its
chrono-based build-time regeneration tool. A hosted compilation/scan regression
exercises the shipped schema and math module; the YARA feature is still available.
