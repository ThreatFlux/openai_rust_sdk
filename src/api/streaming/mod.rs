//! # OpenAI Streaming API Client
//!
//! This module provides a streaming client for the OpenAI API, allowing you to stream
//! responses as they are generated instead of waiting for complete responses.
//!
//! ## Features
//!
//! - **Response Streaming**: Stream chat completions as they are generated
//! - **Function Calling**: Support for streaming function calls and tool use
//! - **Event Processing**: Fine-grained control over stream events
//! - **Channel Support**: Create channel-based streams for async processing
//! - **Error Handling**: Robust error handling for stream interruptions
//!
//! ## Example
//!
//! ```rust,no_run
//! use openai_rust_sdk::api::streaming::StreamingApi;
//! use openai_rust_sdk::models::responses::ResponseRequest;
//! use futures::StreamExt;
//!
//! # tokio_test::block_on(async {
//! let api = StreamingApi::new("your-api-key")?;
//!
//! // Create a simple text stream
//! let mut stream = api.create_text_stream("gpt-4", "Tell me a story").await?;
//!
//! // Process stream chunks
//! while let Some(chunk) = stream.next().await {
//!     let chunk = chunk?;
//!     for choice in chunk.choices {
//!         if let Some(content) = choice.delta.content {
//!             print!("{}", content);
//!         }
//!     }
//! }
//! # Ok::<(), openai_rust_sdk::OpenAIError>(())
//! # });
//! ```

pub mod client;
pub mod function_state;
pub mod helpers;
pub mod processor;
pub mod stream_operations;
pub mod types;
pub mod utilities;

// Re-export main types and functions for convenience
pub use client::StreamingApi;
pub use helpers::{
    chunk_to_events, collect_stream_response, process_stream_event, to_streaming_json,
    ResponseStreamExt,
};
pub use processor::FunctionStreamProcessor;
pub use types::{
    FunctionStream, FunctionStreamEvent, ResponseStream, StreamEventType, StreamProcessingState,
};

// StreamingApi already re-exported above in the main re-exports section
