//! Base HTTP client abstraction for OpenAI API
//!
//! This module provides common HTTP client functionality organized into focused modules:
//! - Core HTTP client implementation and configuration
//! - Request builders split into basic and advanced methods
//! - Response handlers with error conversion
//! - Error handling utilities
//! - Utility functions and validation
//! - Legacy utilities for backward compatibility

// Core modules
pub mod client;
pub mod config;
pub mod error;

// Request handling modules
pub mod advanced_requests;
pub mod basic_requests;
pub mod response_handlers;

// Utility modules
pub mod helpers;
pub mod utilities; // Legacy module for backward compatibility

// Re-export the main client and commonly used items
pub use client::HttpClient;
pub use config::{validate_request, ClientConfig, Validate, DEFAULT_BASE_URL};
pub use error::{map_parse_error, map_request_error};

// Re-export for backward compatibility
pub use utilities::{handle_error_response_with_json, handle_simple_error_response};
