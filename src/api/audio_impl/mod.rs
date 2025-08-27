//! # Audio API
//!
//! This module provides access to OpenAI's Audio API for text-to-speech,
//! speech-to-text transcription, and translation.
//!
//! The module is organized into the following submodules:
//! - `types` - Common types and re-exports
//! - `client` - Main AudioApi client implementation  
//! - `speech` - Text-to-speech functionality
//! - `transcription` - Speech-to-text transcription functionality
//! - `translation` - Audio translation functionality
//! - `utilities` - Helper functions and AudioUtils

/// Common types and re-exports
pub mod types;

/// Main AudioApi client implementation
pub mod client;

/// Text-to-speech functionality
pub mod speech;

/// Speech-to-text transcription functionality
pub mod transcription;

/// Audio translation functionality
pub mod translation;

/// Helper functions and AudioUtils
pub mod utilities;

// Re-export the main client and utility types for public API
pub use client::AudioApi;
pub use utilities::AudioUtils;

// Re-export all types from the models module for convenience
pub use types::{
    AudioFormat, AudioModels, AudioSpeechRequest, AudioSpeechResponse, AudioTranscriptionRequest,
    AudioTranscriptionResponse, AudioTranslationRequest, AudioTranslationResponse, Voice,
};

#[cfg(test)]
pub use types::{SpeechBuilder, TranscriptionBuilder, TranscriptionFormat, TranslationBuilder};
