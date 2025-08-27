//! # Model Requirements
//!
//! Requirements specification for finding suitable models.

use super::types::CompletionType;

/// Requirements for finding suitable models
#[derive(Debug, Clone)]
pub struct ModelRequirements {
    /// Required completion types
    pub completion_types: Vec<CompletionType>,

    /// Minimum context window size needed
    pub min_max_tokens: Option<u32>,

    /// Whether function calling is required
    pub requires_function_calling: bool,

    /// Whether vision capabilities are required
    pub requires_vision: bool,

    /// Whether code interpreter is required
    pub requires_code_interpreter: bool,

    /// Whether to exclude deprecated models
    pub exclude_deprecated: bool,
}

impl Default for ModelRequirements {
    fn default() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            min_max_tokens: None,
            requires_function_calling: false,
            requires_vision: false,
            requires_code_interpreter: false,
            exclude_deprecated: true,
        }
    }
}

impl ModelRequirements {
    /// Create requirements for basic chat
    #[must_use]
    pub fn chat() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            ..Default::default()
        }
    }

    /// Create requirements for function calling
    #[must_use]
    pub fn function_calling() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_function_calling: true,
            ..Default::default()
        }
    }

    /// Create requirements for vision tasks
    #[must_use]
    pub fn vision() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            requires_vision: true,
            ..Default::default()
        }
    }

    /// Create requirements for code tasks
    #[must_use]
    pub fn code() -> Self {
        Self {
            completion_types: vec![CompletionType::Chat, CompletionType::Code],
            requires_code_interpreter: true,
            ..Default::default()
        }
    }

    /// Create requirements for high-context tasks
    #[must_use]
    pub fn high_context(min_tokens: u32) -> Self {
        Self {
            completion_types: vec![CompletionType::Chat],
            min_max_tokens: Some(min_tokens),
            ..Default::default()
        }
    }
}