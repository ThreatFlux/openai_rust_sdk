//! # Models API
//!
//! This module provides access to OpenAI's Models API for listing available models
//! and retrieving detailed information about specific models and their capabilities.
//!
//! This module has been reorganized for better maintainability:
//! - `types` - Type definitions and data structures
//! - `client` - Core client functionality
//! - `operations` - High-level operations and filtering
//! - `analysis` - Advanced analysis and cost estimation
//! - `helpers` - Utility functions and helpers

pub mod analysis;
pub mod client;
pub mod helpers;
pub mod operations;
pub mod types;

// Re-export all public items to maintain API compatibility
pub use analysis::*;
pub use client::*;
pub use helpers::*;
pub use operations::*;
pub use types::*;
