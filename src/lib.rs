//! # OpenAI Rust SDK
//!
//! `openai_rust_sdk` is an unofficial, async, type-safe Rust client for the OpenAI API. The
//! [Responses API](api::responses_v2) is the recommended starting point for new integrations;
//! the crate also covers conversations, streaming and tools, Batch, files, vector stores,
//! media, Realtime, fine-tuning, evals, and administration APIs.
//!
//! This community-maintained crate is not affiliated with, endorsed by, or maintained by OpenAI.
//! The API changes quickly, so consult the dated
//! [coverage matrix](https://github.com/ThreatFlux/openai_rust_sdk/blob/main/docs/api-coverage.md)
//! before depending on a particular endpoint in production.
//!
//! ## Quick start
//!
//! Set `OPENAI_API_KEY` and choose `OPENAI_MODEL` explicitly (for example,
//! [`gpt-5.6-luna`](https://developers.openai.com/api/docs/models/gpt-5.6-luna)), then create a
//! response:
//!
//! ```rust,no_run
//! use openai_rust_sdk::{OpenAIError, from_env, models::CreateResponseRequest};
//!
//! #[tokio::main]
//! async fn main() -> openai_rust_sdk::Result<()> {
//!     let client = from_env()?;
//!     let model = std::env::var("OPENAI_MODEL")
//!         .ok()
//!         .map(|value| value.trim().to_owned())
//!         .filter(|value| !value.is_empty())
//!         .ok_or_else(|| {
//!             OpenAIError::InvalidRequest(
//!                 "OPENAI_MODEL must be set to a non-empty model ID".to_owned(),
//!             )
//!         })?;
//!     let request = CreateResponseRequest::new_text(
//!         model,
//!         "Explain Rust's ownership model in one concise sentence.",
//!     );
//!
//!     let response = client.create_response_v2(&request).await?;
//!     println!("{}", response.output_text());
//!     Ok(())
//! }
//! ```
//!
//! `from_env` also accepts the optional `OPENAI_BASE_URL` environment variable. A custom base URL
//! must be an API origin without `/v1` or a trailing slash. The API key is sent as a bearer token
//! to that origin, so only configure endpoints you trust.
//!
//! ## API lifecycle
//!
//! Responses and Conversations are the modern integration path. Assistants, Threads, and Runs
//! remain available for migration, but OpenAI has deprecated the Assistants API and announced its
//! shutdown for August 26, 2026. See OpenAI's
//! [migration guide](https://developers.openai.com/api/docs/guides/migrate-to-responses).
//!
//! ## Cargo features
//!
//! - `default`: the core OpenAI API client.
//! - `yara`: local validation and CLI tooling for the optional YARA-X Batch API example.
//! - `testing`: reserved compatibility feature; currently adds no dependencies.
//! - `full`: all optional capabilities; currently enables `testing` and `yara`.
//!
//! See the
//! [YARA-X Batch example](https://github.com/ThreatFlux/openai_rust_sdk/blob/main/docs/examples/batch-yara-x.md)
//! for exact CLI commands and security considerations.
//!
//! ## Reliability
//!
//! The shared HTTP client does not currently apply automatic retries or expose a global request
//! timeout. Applications should add workload-appropriate timeouts, exponential backoff with
//! jitter, and idempotency handling. More details are in the
//! [configuration guide](https://github.com/ThreatFlux/openai_rust_sdk/blob/main/docs/configuration.md).
//!
//! Complete, runnable programs live in the repository's
//! [examples directory](https://github.com/ThreatFlux/openai_rust_sdk/tree/main/examples), starting
//! with `quickstart.rs` and `responses_api.rs`.

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
// Allow clippy warnings that are cosmetic or would require extensive refactoring
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::return_self_not_must_use)]
#![allow(clippy::use_self)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::float_cmp)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::case_sensitive_file_extension_comparisons)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::match_same_arms)]
#![allow(clippy::unused_self)]
#![allow(clippy::unused_async)]
#![allow(clippy::unnecessary_wraps)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::future_not_send)]
#![allow(clippy::significant_drop_tightening)]
#![allow(clippy::significant_drop_in_scrutinee)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::needless_raw_string_hashes)]
#![allow(clippy::redundant_closure_for_method_calls)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::needless_pass_by_ref_mut)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::single_char_pattern)]
#![allow(clippy::manual_string_new)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::redundant_clone)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::multiple_crate_versions)]
#![allow(unused_imports)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Common macros for reducing code duplication
#[macro_use]
pub mod macros;

// Re-export shortened serde traits for use across the codebase
pub use macros::{De, Ser};

/// `OpenAI` API functionality
pub mod api;
/// Function and schema builders
pub mod builders;
/// Client implementations and builders
pub mod client;
/// Constants used throughout the SDK
pub mod constants;
/// Error types and handling
pub mod error;
/// Helper utilities to reduce code duplication
pub mod helpers;
/// Data models and types
pub mod models;
/// Prompt engineering utilities and builders
pub mod prompt_engineering;
/// JSON Schema utilities
pub mod schema;
/// Utilities for the optional YARA-X Batch API example
pub mod testing;

// Re-export main OpenAI API types for convenience
pub use api::batch::BatchApi;
pub use api::streaming::StreamingApi;
pub use api::{
    AdminApi, AssistantsApi, ConversationsApi, CustomToolsApi, EvalsApi, FineTuningApi,
    FunctionsApi, RealtimeAudioApi, ResponsesApi, RunsApi, SkillsApi, ThreadsApi, UploadsApi,
    VectorStoresApi, VideosApi,
};
pub use builders::{FunctionBuilder, ObjectSchemaBuilder};
pub use client::{ChatBuilder, OpenAIClient, from_env, from_env_with_base_url};
pub use error::{OpenAIError, Result};
pub use models::realtime::*;
pub use models::responses_v2::{
    CreateResponseRequest, InputTokenCountResponse, ResponseStreamEvent,
};
pub use models::{assistants::*, functions::*, responses::*};
pub use prompt_engineering::{
    Example, PromptBuilder, PromptPatterns, PromptTemplateBuilder, XmlContentBuilder,
};
pub use schema::{EnhancedSchemaBuilder, JsonSchema, SchemaBuilder};

// Re-export Batch example generation
pub use testing::batch_generator::BatchJobGenerator;

// Re-export optional case-study validation when the feature is enabled
#[cfg(feature = "yara")]
pub use testing::{
    test_cases::YaraTestCases,
    yara_validator::{ValidationError, ValidationResult, YaraValidator},
};
