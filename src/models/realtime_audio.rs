//! # Real-time Audio Models
//!
//! Data structures for OpenAI's real-time audio API with WebRTC support,
//! including session management, events, and audio streaming.
//!
//! This module has been refactored from a single large file (1264 lines) into
//! focused submodules for better maintainability:
//!
//! - Session Configuration: Audio formats, voices, session setup
//! - Event Types: WebSocket events for real-time communication  
//! - Conversation Types: Message content, roles, function calls
//! - Audio Processing: WebRTC stats, voice activity, audio buffers  
//! - Response Types: API responses, errors, usage statistics

#[path = "realtime_audio/session_config.rs"]
pub mod session_config;

#[path = "realtime_audio/event_types.rs"]
pub mod event_types;

#[path = "realtime_audio/conversation_types.rs"]
pub mod conversation_types;

#[path = "realtime_audio/audio_processing.rs"]
pub mod audio_processing;

#[path = "realtime_audio/response_types.rs"]
pub mod response_types;

// Re-export all public types for backward compatibility
pub use audio_processing::*;
pub use conversation_types::*;
pub use event_types::*;
pub use response_types::*;
pub use session_config::*;
