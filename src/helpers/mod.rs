//! Helper modules for reducing code duplication
//!
//! This module contains various helper functions and utilities that are
//! used across the codebase to eliminate duplicate patterns and improve
//! code maintainability.

pub mod file_operations;

// Re-export commonly used functions for convenience
pub use file_operations::{
    read_bytes, read_bytes_sync, read_string, read_string_sync, write_bytes, write_bytes_sync,
    write_string, write_string_sync,
};
