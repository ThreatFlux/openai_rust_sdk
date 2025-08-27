//! Legacy utilities module - deprecated, use specific modules instead

// Re-export all utility functions from their new locations for backward compatibility
pub use crate::api::base::config::{validate_request, Validate};
pub use crate::api::base::error::{
    handle_error_response_with_json, handle_simple_error_response, map_parse_error,
    map_request_error,
};

// This file is kept for backward compatibility
// All new code should import from the specific modules:
// - crate::api::base::config for validation traits and request validation
// - crate::api::base::error for error handling utilities
// - crate::api::base::helpers for general utility functions
