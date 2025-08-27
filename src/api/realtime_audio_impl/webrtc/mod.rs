//! # WebRTC Module
//!
//! WebRTC connection management for real-time audio.

/// WebRTC connection creation and configuration
pub mod connection;
/// WebRTC session setup and handlers
pub mod setup;

// Re-export public types
pub use connection::*;
