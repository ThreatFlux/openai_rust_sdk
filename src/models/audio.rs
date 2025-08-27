//! # Audio Models
//!
//! Data structures for the OpenAI Audio API including text-to-speech,
//! speech-to-text transcription, and translation endpoints.
//!
//! This module has been restructured for better organization:
//! - `types` - Core types and enums
//! - `requests` - Request structures
//! - `responses` - Response structures
//! - `builders` - Builder patterns
//! - `models` - Model constants

#[path = "audio_impl/mod.rs"]
mod audio_impl;

pub use audio_impl::*;
