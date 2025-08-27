//! # Audio API Types
//!
//! Common types, re-exports and type aliases for the Audio API module.

// Re-export all types from the models module
pub use crate::models::audio::{
    AudioFormat, AudioModels, AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest,
    AudioTranscriptionResponse, AudioTranslationRequest, AudioTranslationResponse, Voice,
};

#[cfg(test)]
pub use crate::models::audio::{
    SpeechBuilder, TranscriptionBuilder, TranscriptionFormat, TranslationBuilder,
};

// Re-export common dependencies
pub use crate::api::base::HttpClient;
pub use crate::api::common::ApiClientConstructors;
pub use crate::api::shared_utilities::{
    AudioResponseHandler, FormBuilder, MultipartRequestExecutor, RequestValidator,
};
pub use crate::error::{OpenAIError, Result};
pub use bytes::Bytes;
pub use std::path::Path;
pub use tokio::fs;
pub use tokio_stream::StreamExt;
