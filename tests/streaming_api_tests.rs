#![allow(clippy::pedantic, clippy::nursery)]
//! Comprehensive tests for the Streaming API
//!
//! This test suite covers core functionality of the Streaming API including:
//! - Stream API creation and configuration
//! - Request building for streaming
//! - Function calling configuration
//! - Error handling
//! - Streaming utilities and helpers
//!
//! The tests are organized into focused modules for better maintainability:
//! - `api_creation`: API creation and configuration tests
//! - `requests`: Request building and streaming configuration tests
//! - `function_calls`: Function calling functionality tests
//! - `tools`: Tool configuration and serialization tests
//! - `errors`: Error handling tests
//! - `messages`: Message creation and handling tests
//! - `builders`: Request builder pattern tests
//! - `edge_cases`: Edge cases and boundary condition tests
//! - `integration`: Integration test preparation

mod streaming;

// Re-export all test modules to maintain the same test structure
pub use streaming::{
    api_creation, builders, edge_cases, errors, function_calls, integration, messages, requests,
    test_helpers, tools,
};
