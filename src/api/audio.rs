//! # Audio API
//!
//! This module provides access to OpenAI's Audio API for text-to-speech,
//! speech-to-text transcription, and translation.
//!
//! The functionality has been split into multiple modules for better organization:
//! - Text-to-speech (speech generation)
//! - Speech-to-text (transcription)
//! - Audio translation
//! - Utilities and helper functions

// Import the audio implementation submodule and re-export everything
#[path = "audio_impl/mod.rs"]
mod audio_impl;

// Re-export all public items from the audio implementation submodule
pub use audio_impl::*;
