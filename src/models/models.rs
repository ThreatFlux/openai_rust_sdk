//! # Models API
//!
//! Data structures for the OpenAI Models API that provides information about
//! available models, their capabilities, and permissions.
//!
//! This module has been restructured for better organization:
//! - `types` - Core model data structures
//! - `enums` - Model categorization enumerations  
//! - `implementations` - Method implementations for model types
//! - `helpers` - Utility functions for model analysis

#[path = "models_modular/mod.rs"]
mod models_module;

pub use models_module::*;
