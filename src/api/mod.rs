/// Common utilities for API clients
pub mod common;
/// Shared utilities to reduce code duplication across API clients
pub mod shared_utilities;

/// Assistants API for AI assistant creation and management
pub mod assistants;
/// Audio API for text-to-speech, transcription, and translation
pub mod audio;
/// Base HTTP client abstraction for all API clients
pub mod base;
/// Batch API implementation for asynchronous processing
pub mod batch;
/// Common API client trait and implementations
pub mod client;
/// Container management API for Code Interpreter
pub mod containers;
/// Custom tools support
pub mod custom_tools;
/// Embeddings API for vector representations
pub mod embeddings;
/// Files API for file upload, management, and retrieval
pub mod files;
/// Fine-tuning API for custom model training and management
pub mod fine_tuning;
/// Function calling API implementation
pub mod functions;
/// GPT-5 specific API features
pub mod gpt5;
/// Images API for DALL-E image generation, editing, and variations
pub mod images;
/// Models API for listing and retrieving model information
pub mod models;
/// Moderations API for content policy classification
pub mod moderations;
/// Real-time Audio API for WebRTC-based audio streaming
pub mod realtime_audio;
/// Response API implementation
pub mod responses;
/// Helper functions for responses API
mod responses_helpers;
/// Runs API for assistant execution and run steps management
pub mod runs;
/// Streaming API implementation
pub mod streaming;
/// Helper functions for streaming API
mod streaming_helpers;
/// Threads API for conversation thread and message management
pub mod threads;
/// Vector stores API for RAG and knowledge management
pub mod vector_stores;

// Re-export specific items to avoid naming conflicts
pub use assistants::*;
// Audio exports - explicitly list to avoid conflict with threads::types
pub use audio::types as audio_types;
pub use audio::{client::AudioApi, speech, transcription, translation, utilities};
pub use batch::{BatchApi, BatchReport, BatchStatus, YaraProcessor};
pub use containers::*;
pub use custom_tools::*;
pub use embeddings::*;
pub use files::*;
pub use fine_tuning::*;
// Functions exports - be explicit to avoid conflicts
pub use functions::{
    ConversationState, FunctionCallEvent, FunctionConfig, FunctionResponseResult, FunctionsApi,
};
pub use gpt5::*;
pub use images::*;
// Models exports - be explicit to avoid conflicts
pub use models::ModelsApi;
pub use moderations::ModerationsApi;
pub use realtime_audio::*;
pub use responses::*;
pub use runs::*;
pub use streaming::{FunctionStream, ResponseStream, ResponseStreamExt, StreamingApi};
// Threads exports - explicitly list to avoid conflict with audio::types
pub use threads::types as thread_types;
pub use threads::{client::ThreadsApi, files as thread_files, messages, operations};
pub use vector_stores::*;
