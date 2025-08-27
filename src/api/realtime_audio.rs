//! # Real-time Audio API
//!
//! This module provides WebRTC-based real-time audio streaming capabilities
//! for OpenAI's real-time audio API, supporting bidirectional audio streaming,
//! voice activity detection, and low-latency communication.
//!
//! This module has been restructured for better organization:
//! - `client` - Core client functionality
//! - `config` - Configuration structures
//! - `session` - Session management
//! - `webrtc` - WebRTC connection management
//! - `vad` - Voice activity detection
//! - `audio_processor` - Audio processing and effects
//! - `builders` - Builder patterns for session creation

#[path = "realtime_audio_impl/mod.rs"]
mod realtime_audio_impl;

pub use realtime_audio_impl::*;
