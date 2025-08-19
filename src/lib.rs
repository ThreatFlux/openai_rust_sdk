//! # Batch `OpenAI` SDK with YARA Rule Validation and `OpenAI` API Client
//!
//! This crate provides a comprehensive SDK for integrating `OpenAI`'s API with YARA rule validation,
//! including support for chat completions, streaming responses, and automated testing.

#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
//!
//! ## Features
//!
//! - **`OpenAI` API Client**: Full-featured client with streaming support
//! - **Chat Completions**: Support for role-based conversations and prompt templates
//! - **Streaming Support**: Real-time streaming responses with Server-Sent Events
//! - **YARA Rule Validation**: Complete validation pipeline using the yara-x engine
//! - **`OpenAI` Batch API Integration**: Generate and manage batch jobs for rule creation
//! - **Comprehensive Testing**: Unit tests, integration tests, and performance benchmarks
//! - **Error Handling**: Robust error handling with detailed validation results
//! - **Performance Metrics**: Compilation time, memory usage, and rule complexity analysis
//!
//! ## Quick Start - `OpenAI` API
//!
//! ```rust,no_run
//! use openai_rust_sdk::{OpenAIClient, ChatBuilder};
//!
//! # tokio_test::block_on(async {
//! let client = OpenAIClient::new("your-api-key")?;
//!
//! // Simple text generation
//! let response = client.generate_text("gpt-4", "Hello, world!").await?;
//! println!("Response: {}", response);
//!
//! // Chat conversation
//! let conversation = ChatBuilder::new()
//!     .developer("You are a helpful assistant")
//!     .user("What is Rust?");
//!
//! let response = client.chat("gpt-4", conversation).await?;
//! println!("Chat response: {}", response);
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # }).unwrap();
//! ```
//!
//! ## Quick Start - YARA Validation
//!
//! ```rust,ignore
//! # #[cfg(feature = "yara")]
//! use openai_rust_sdk::testing::YaraValidator;
//!
//! # #[cfg(feature = "yara")]
//! # tokio_test::block_on(async {
//! let validator = YaraValidator::new();
//! let rule = r#"
//! rule test_rule {
//!     strings:
//!         $hello = "Hello World"
//!     condition:
//!         $hello
//! }
//! "#;
//!
//! let result = validator.validate_rule(rule)?;
//! println!("Rule is valid: {}", result.is_valid);
//! # Ok::<(), anyhow::Error>(())
//! # }).unwrap();
//! ```
//!
//! ## Modules
//!
//! - [`client`]: Main `OpenAI` client with all API functionality
//! - [`api`]: Individual API modules (responses, streaming)
//! - [`models`]: Data models for requests and responses
//! - [`testing`]: YARA validation and testing functionality
//! - [`error`]: Error types and handling
//!
//! ## Examples
//!
//! See the `examples/` directory for complete usage examples:
//! - `chat_completion.rs`: Chat completions and streaming examples
//! - `full_integration.rs`: Complete workflow demonstration
//! - `basic_validation.rs`: Simple rule validation
//! - `streaming_demo.rs`: Streaming API integration
//! - `error_handling.rs`: Error handling patterns

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

/// Common macros for reducing code duplication
#[macro_use]
pub mod macros;

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
/// Data models and types
pub mod models;
/// Prompt engineering utilities and builders
pub mod prompt_engineering;
/// JSON Schema utilities
pub mod schema;
/// YARA testing and validation functionality
pub mod testing;

// Re-export main OpenAI API types for convenience
pub use api::{
    AssistantsApi, BatchApi, CustomToolsApi, FineTuningApi, FunctionsApi, RealtimeAudioApi,
    ResponsesApi, RunsApi, StreamingApi, ThreadsApi, VectorStoresApi,
};
pub use builders::{FunctionBuilder, ObjectSchemaBuilder};
pub use client::{from_env, from_env_with_base_url, ChatBuilder, OpenAIClient};
pub use error::{OpenAIError, Result};
pub use models::{assistants::*, functions::*, responses::*};
pub use prompt_engineering::{
    Example, PromptBuilder, PromptPatterns, PromptTemplateBuilder, XmlContentBuilder,
};
pub use schema::{EnhancedSchemaBuilder, JsonSchema, SchemaBuilder};

// Re-export testing functionality
pub use testing::batch_generator::BatchJobGenerator;

// Re-export YARA testing functionality when feature is enabled
#[cfg(feature = "yara")]
pub use testing::{
    test_cases::YaraTestCases,
    yara_validator::{ValidationError, ValidationResult, YaraValidator},
};
